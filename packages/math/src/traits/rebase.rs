use crate::{
    common::{checked_add, checked_sub, exp10, muldiv},
    U256,
};
use btr_macros::borsh_serde;
use cosmwasm_std::{StdResult, Uint256};

pub trait Rebase {
    fn elastic_uint256(&self) -> Uint256;
    fn base_uint256(&self) -> Uint256;
    fn elastic(&self) -> U256;
    fn base(&self) -> U256;
    fn set_elastic(&mut self, elastic: U256);
    fn set_base(&mut self, base: U256);
    fn into_rebase(&self) -> SimpleRebase {
        SimpleRebase::new(self.elastic(), self.base())
    }
    /// Calculates the base value in relationship to `elastic` and self
    fn to_base(&self, elastic: impl Into<U256> + Copy, round_up: bool) -> StdResult<U256> {
        let elastic = elastic.into();
        let mut base;

        // Use virtual offset approach in YieldBox to enforce a base conversion rate.
        // Because we want to support at most 18 decimal fixed point math, we set the ratio to 1 : 1e18.
        let total_shares = self.base() + exp10(18);
        let total_amount = self.elastic() + U256::ONE;

        base = muldiv(elastic, total_shares, total_amount)?;
        if round_up && muldiv(base, total_amount, total_shares)? < elastic {
            base += U256::ONE;
        }
        Ok(base)
    }

    /// Calculates the elastic value in relationship to `base` and self
    fn to_elastic(&self, base: impl Into<U256> + Copy, round_up: bool) -> StdResult<U256> {
        let base = base.into();
        let mut elastic;

        // Use virtual offset approach in YieldBox to enforce a base conversion rate.
        // Because we want to support at most 18 decimal fixed point math, we set the ratio to 1 : 1e18.
        let total_shares = self.base() + exp10(18);
        let total_amount = self.elastic() + U256::ONE;

        elastic = muldiv(base, total_amount, total_shares)?;
        if round_up && muldiv(elastic, total_shares, total_amount)? < base {
            elastic += U256::ONE;
        }
        Ok(elastic)
    }

    /// Add `elastic` to `self` and update `total.base`
    fn add_elastic(
        &mut self,
        elastic: impl Into<U256> + Copy,
        round_up: bool,
    ) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        let elastic: U256 = elastic.into();
        self.set_elastic(checked_add(self.elastic(), elastic)?);
        self.set_base(checked_add(self.base(), base)?);
        Ok((self, base))
    }

    /// Sub `elastic` from `self` and update `total.base`
    fn sub_elastic(
        &mut self,
        elastic: impl Into<U256> + Copy,
        round_up: bool,
    ) -> StdResult<(&mut Self, U256)> {
        let base = self.to_base(elastic, round_up)?;
        let elastic: U256 = elastic.into();
        self.set_elastic(checked_sub(self.elastic(), elastic)?);
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        self.set_base(self.base() - base);
        Ok((self, base))
    }

    /// Add `base` to `total` and update `self.elastic()`
    fn add_base(
        &mut self,
        base: impl Into<U256> + Copy,
        round_up: bool,
    ) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.set_elastic(checked_add(self.elastic(), elastic)?);
        let base: U256 = base.into();
        self.set_base(checked_add(self.base(), base)?);
        Ok((self, elastic))
    }

    /// Sub `base` from `total` and update `self.elastic()`
    fn sub_base(
        &mut self,
        base: impl Into<U256> + Copy,
        round_up: bool,
    ) -> StdResult<(&mut Self, U256)> {
        let elastic = self.to_elastic(base, round_up)?;
        self.set_elastic(checked_sub(self.elastic(), elastic)?);
        // The amount we are subtracting from elastic and base are proportional in this function
        // so if we pass the checked_sub above, we don't need to check again.
        let base: U256 = base.into();
        self.set_base(self.base() - base);
        Ok((self, elastic))
    }
}

#[borsh_serde]
#[derive(Default)]
pub struct SimpleRebase {
    pub elastic: U256,
    pub base: U256,
}

impl SimpleRebase {
    pub fn new(elastic: U256, base: U256) -> Self {
        Self { elastic, base }
    }
}

impl Rebase for SimpleRebase {
    fn elastic_uint256(&self) -> Uint256 {
        self.elastic.into()
    }

    fn base_uint256(&self) -> Uint256 {
        self.base.into()
    }

    fn elastic(&self) -> U256 {
        self.elastic
    }

    fn base(&self) -> U256 {
        self.base
    }

    fn set_elastic(&mut self, elastic: U256) {
        self.elastic = elastic;
    }

    fn set_base(&mut self, base: U256) {
        self.base = base;
    }
}
