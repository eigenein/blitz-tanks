use std::collections::HashMap;

use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::{
        item_item::{FitParams, Model, PredictParams},
        metrics::ReciprocalRank,
    },
};

const PROGRESS_TEMPLATE: &str = "{elapsed} {per_sec} {wide_bar} {pos}/{len} {eta}";

pub fn search(
    votes: &mut [Vote],
    n_partitions: usize,
    test_proportion: f64,
    params: impl IntoIterator<Item = (FitParams, PredictParams)>,
) -> Option<(ReciprocalRank, FitParams, PredictParams)> {
    info!(
        n_votes = votes.len(),
        n_partitions, test_proportion, "🧪 Searching across the parameter space…",
    );
    params
        .into_iter()
        .try_progress()
        .unwrap()
        .with_style(ProgressStyle::with_template(PROGRESS_TEMPLATE).unwrap())
        .map(|(fit_params, predict_params)| {
            (
                fit_and_cross_validate(
                    votes,
                    n_partitions,
                    test_proportion,
                    &fit_params,
                    &predict_params,
                ),
                fit_params,
                predict_params,
            )
        })
        .fold(None, |current, (reciprocal_rank, fit_params, predict_params)| {
            if current.as_ref().map_or(true, |(current_reciprocal_rank, _, _)| {
                reciprocal_rank > *current_reciprocal_rank
            }) {
                info!(
                    %reciprocal_rank,
                    disable_damping = fit_params.disable_damping,
                    n_neighbors = predict_params.n_neighbors,
                    include_negative = predict_params.include_negative,
                    "🎉 Improved",
                );
                Some((reciprocal_rank, fit_params, predict_params))
            } else {
                current
            }
        })
}

pub fn fit_and_cross_validate(
    votes: &mut [Vote],
    n_partitions: usize,
    test_proportion: f64,
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> ReciprocalRank {
    let split_index = (votes.len() as f64 * test_proportion) as usize;

    (0..n_partitions)
        .map(|_| {
            fastrand::shuffle(votes);
            fit_and_validate(
                &votes[split_index..],
                &votes[..split_index],
                fit_params,
                predict_params,
            )
        })
        .sum::<ReciprocalRank>()
        / n_partitions
}

pub fn fit_and_validate(
    train: &[Vote],
    test: &[Vote],
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> ReciprocalRank {
    let model = Model::fit(train, fit_params);

    let train_ratings: HashMap<u32, HashMap<i32, Rating>> = train
        .iter()
        .into_group_map_by(|vote| vote.account_id)
        .into_iter()
        .map(|(account_id, train_votes)| {
            (
                account_id,
                train_votes.into_iter().map(|vote| (vote.tank_id, vote.rating)).collect(),
            )
        })
        .collect();

    let test = test.iter().into_group_map_by(|vote| vote.account_id);
    let n_test_accounts = test.len();
    test.into_iter()
        .filter_map(|(account_id, test_votes)| {
            let Some(train_ratings) = train_ratings.get(&account_id) else { return None };
            if !test_votes.iter().any(|vote| vote.rating == Rating::Like) {
                // Can't even calculate the metrics in this case.
                return None;
            }
            let predictions = model
                .predict_many(
                    test_votes.iter().map(|vote| vote.tank_id),
                    train_ratings,
                    predict_params,
                )
                .zip(test_votes.iter().copied())
                .collect_vec();
            let n_predictions = predictions.len();
            debug!(
                account_id,
                n_train_ratings = train_ratings.len(),
                n_test_votes = test_votes.len(),
                n_predictions,
            );
            let reciprocal_rank = predictions
                .iter()
                .sorted_unstable_by_key(|(prediction, _)| prediction)
                .map(|(_, vote)| vote.rating)
                .collect::<ReciprocalRank>();
            Some(reciprocal_rank)
        })
        .sum::<ReciprocalRank>()
        / n_test_accounts
}
