use std::collections::HashMap;

use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::{
        item_item::{FitParams, Model, PredictParams},
        metrics::Metrics,
    },
};

pub fn fit_and_cross_validate(
    votes: &mut [Vote],
    n_partitions: usize,
    proportion: f64,
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> Metrics {
    let split_index = (votes.len() as f64 * proportion) as usize;
    info!(
        n_partitions,
        split_index,
        ?fit_params,
        ?predict_params,
        "🧪 Fitting and validating…"
    );

    (0..n_partitions)
        .progress()
        .map(|_| {
            fastrand::shuffle(votes);
            fit_and_validate(
                &votes[split_index..],
                &votes[..split_index],
                fit_params,
                predict_params,
            )
        })
        .sum::<Metrics>()
        / n_partitions as f64
}

pub fn fit_and_validate(
    train: &[Vote],
    test: &[Vote],
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> Metrics {
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
    let n_test_accounts = test.len() as f64;
    test.into_iter()
        .filter_map(|(account_id, test_votes)| {
            let Some(train_ratings) = train_ratings.get(&account_id) else { return None };
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
            Some(Metrics::from(predictions))
        })
        .sum::<Metrics>()
        / n_test_accounts
}
