//! Benchmarking setup for pallet-betting

use super::*;

#[allow(unused)]
use crate::Pallet as Betting;
use frame_benchmarking::{account, benchmarks, whitelisted_caller};
use frame_system::RawOrigin;

fn create_match<T: Config>(result: Option<MatchResult>) -> T::AccountId {
    let caller: T::AccountId = account("creator", 0, 0);
    let start = T::BlockNumber::from(5u32);
    let length = T::BlockNumber::from(5u32);

    let betting_match = Match {
        start,
        length,
        team1: <BoundedVec<_, T::MaxTeamNameLength>>::try_from("team1".as_bytes().to_vec())
            .unwrap(),
        team2: <BoundedVec<_, T::MaxTeamNameLength>>::try_from("team2".as_bytes().to_vec())
            .unwrap(),
        result,
        bets: Default::default(),
    };

    let match_hash = Betting::<T>::get_match_hash(betting_match.clone());

    <MatchHashes<T>>::insert(&match_hash, caller.clone());
    <Matches<T>>::insert(&caller, betting_match);

    caller
}

fn add_bet<T: Config>(user: &'static str, match_id: AccountIdOf<T>, a: BalanceOf<T>, r: MatchResult) {
    let caller = account(user, 0, 0);
    T::Currency::make_free_balance_be(&caller, T::Currency::minimum_balance() * 10u32.into());
    let origin = <T::RuntimeOrigin>::from(RawOrigin::Signed(caller));
    let _ = Betting::<T>::bet(origin, match_id, a.into(), r);
}

benchmarks! {
    create_match_to_bet {
        let caller: T::AccountId = whitelisted_caller();
        let team1 = "team1".as_bytes().to_vec();
        let team2 = "team2".as_bytes().to_vec();
        let start = T::BlockNumber::from(10u32);
        let length = T::BlockNumber::from(10u32);
    }: _(RawOrigin::Signed(caller.clone()), team1, team2, start, length)
    verify {
        assert!(Matches::<T>::contains_key(&caller));
    }

    bet {
        let match_id = create_match::<T>(None);
        let caller: T::AccountId = whitelisted_caller();
        T::Currency::make_free_balance_be(&caller, T::Currency::minimum_balance() * 10u32.into());
        let amount = BalanceOf::<T>::from(T::Currency::minimum_balance());
        let result = MatchResult::Draw;
    }: _(RawOrigin::Signed(caller.clone()), match_id.clone(), amount, result)
    verify {
        let m = Matches::<T>::get(&match_id).unwrap();
        assert_eq!(m.bets.len(), 1);
    }

    set_result {
        let match_id = create_match::<T>(None);
        frame_system::Pallet::<T>::set_block_number(15u32.into());
        let result = MatchResult::Team1Victory;
    }: _(RawOrigin::Root, match_id.clone(), result)
    verify {
        let m = Matches::<T>::get(&match_id).unwrap();
        assert_eq!(m.result, Some(MatchResult::Team1Victory));
    }

    distribute_winnings {
        let match_id = create_match::<T>(Some(MatchResult::Team1Victory));
        frame_system::Pallet::<T>::set_block_number(15u32.into());
        let result = MatchResult::Team1Victory;
        add_bet::<T>("user1", match_id.clone(), T::Currency::minimum_balance(), MatchResult::Team1Victory);
        add_bet::<T>("user2", match_id.clone(), T::Currency::minimum_balance() * 2u32.into(), MatchResult::Team2Victory);
        add_bet::<T>("user3", match_id.clone(), T::Currency::minimum_balance() * 3u32.into(), MatchResult::Draw);
        add_bet::<T>("user4", match_id.clone(), T::Currency::minimum_balance() * 4u32.into(), MatchResult::Draw);
        add_bet::<T>("user5", match_id.clone(), T::Currency::minimum_balance() * 5u32.into(), MatchResult::Team1Victory);
    }: _(RawOrigin::Signed(match_id.clone()))
    verify {
        assert_eq!(Matches::<T>::contains_key(&match_id), false);
    }

    impl_benchmark_test_suite!(Betting, crate::mock::new_test_ext(), crate::mock::Test);
}
