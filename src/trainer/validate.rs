use std::collections::HashMap;

use indicatif::ProgressIterator;
use itertools::Itertools;

use crate::{
    models::{rating::Rating, vote::Vote},
    prelude::*,
    trainer::item_item::{FitParams, Model, PredictParams},
};

pub fn fit_and_cross_validate(
    votes: &mut [Vote],
    n: usize,
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> (f64,) {
    let split_index = votes.len() / n;
    info!(n, split_index, ?fit_params, ?predict_params, "🧪 Fitting and validating…");

    let (sum_mrr,) = (0..n)
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
        .fold((0.0,), |(sum_mrr,), (mrr,)| (sum_mrr + mrr,));

    (sum_mrr / n as f64,)
}

pub fn fit_and_validate(
    train: &[Vote],
    test: &[Vote],
    fit_params: &FitParams,
    predict_params: &PredictParams,
) -> (f64,) {
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

    let test = test.iter().into_group_map_by(|vote| vote.account_id).into_iter();
    let n_test_accounts = test.len() as f64;
    let (sum_reciprocal_rank,) = test
        .filter_map(|(account_id, test_votes)| {
            let Some(train_ratings) = train_ratings.get(&account_id) else { return None };
            let test_ratings: HashMap<i32, Rating> =
                test_votes.iter().map(|vote| (vote.tank_id, vote.rating)).collect();
            let predictions =
                model.predict_many(test_ratings.keys().copied(), train_ratings, predict_params);
            let first_good_prediction = predictions
                .iter()
                .enumerate()
                .find(|(_, (tank_id, _))| test_ratings.get(tank_id).copied() == Some(Rating::Like));
            debug!(
                account_id,
                n_train_ratings = train_ratings.len(),
                n_test_ratings = test_ratings.len(),
                n_predictions = predictions.len(),
                ?first_good_prediction,
            );
            match first_good_prediction {
                Some((rank, _)) => Some((1.0 / (rank + 1) as f64,)),
                None => Some((0.0,)),
            }
        })
        .fold((0.0,), |(sum,), (reciprocal_rank,)| (sum + reciprocal_rank,));

    let mean_reciprocal_rank = sum_reciprocal_rank / n_test_accounts;
    (mean_reciprocal_rank,)
}
