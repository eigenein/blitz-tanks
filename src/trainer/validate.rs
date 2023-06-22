use std::collections::HashMap;

use indicatif::{ProgressIterator, ProgressStyle};
use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::{
        item_item::{Model, Params},
        metrics::{Mean, ReciprocalRank},
    },
};

const PROGRESS_TEMPLATE: &str = "{elapsed} {per_sec} {wide_bar} {pos}/{len} {eta}";

pub fn search(
    votes: &[Vote],
    n_partitions: usize,
    test_proportion: f64,
    params: impl IntoIterator<Item = Params>,
) -> ReciprocalRank {
    info!(
        n_votes = votes.len(),
        n_partitions, test_proportion, "🧪 Searching across the parameter space…",
    );
    params
        .into_iter()
        .try_progress()
        .unwrap()
        .with_style(ProgressStyle::with_template(PROGRESS_TEMPLATE).unwrap())
        .map(|params| {
            (fit_and_cross_validate(votes, n_partitions, test_proportion, &params), params)
        })
        .fold(ReciprocalRank::default(), |current_mrr, (new_mrr, new_params)| {
            if new_mrr > current_mrr {
                info!(
                    %new_mrr,
                    enable_damping = new_params.enable_damping,
                    n_neighbors = new_params.n_neighbors,
                    include_negative = new_params.include_negative,
                    "🎉 Improved",
                );
                new_mrr
            } else {
                current_mrr
            }
        })
}

pub fn fit_and_cross_validate(
    votes: &[Vote],
    n_partitions: usize,
    test_proportion: f64,
    params: &Params,
) -> ReciprocalRank {
    (0..n_partitions)
        .map(|_| {
            let (train_set, test_set): (Vec<&Vote>, Vec<&Vote>) =
                votes.iter().partition(|_| fastrand::f64() > test_proportion);
            fit_and_validate(&train_set, &test_set, params)
        })
        .collect::<Mean<ReciprocalRank>>()
        .0
}

pub fn fit_and_validate<'a>(
    train: &'a [&'a Vote],
    test: &'a [&'a Vote],
    params: &Params,
) -> ReciprocalRank {
    let model = Model::fit(train.iter().copied(), params);

    let train_ratings: HashMap<u32, HashMap<u16, Rating>> = train
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

    test.iter()
        .into_group_map_by(|vote| vote.account_id)
        .into_iter()
        .filter_map(|(account_id, test_votes)| {
            let Some(train_ratings) = train_ratings.get(&account_id) else {
                // No train ratings for this account, can't calculate the metrics.
                return None
            };
            let predictions = model
                .predict_many(test_votes.iter().map(|vote| vote.tank_id), train_ratings)
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
        .collect::<Mean<ReciprocalRank>>()
        .0
}
