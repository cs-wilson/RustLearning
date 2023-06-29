/*实现一个存证模块*/
// 1. 存证模块的功能
// 2. 存证模块的存储项
// 3. 存证模块的事件
// 4. 存证模块的错误
// 5. 存证模块的函数
// 6. 存证模块的单元测试

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
pub use weights::WeightInfo;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

pub mod weights;

#[frame_support::pallet]
pub mod pallet {
	pub use frame_support::pallet_prelude::*;
	pub use frame_system::pallet_prelude::*;
	pub use sp_std::prelude::*;
	use super::WeightInfo;

	// 数据结构
	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::config]
	pub trait Config: frame_system::Config {
		#[pallet::constant]
		type MaxClaimLength: Get<u32>; // 最大长度
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>; // 事件
		type WeightInfo: WeightInfo; // 权重
	}

	// 数据结构
	#[pallet::storage]
	pub type Proofs<T: Config> = StorageMap< _, Blake2_128Concat, BoundedVec<u8, T::MaxClaimLength>, (T::AccountId, T::BlockNumber)>;

	//事件
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ClaimCreated(T::AccountId, Vec<u8>),
		ClaimRevoked(T::AccountId, Vec<u8>),
		ClaimTransferred(T::AccountId, T::AccountId, Vec<u8>),
	}

	// 错误
	#[pallet::error]
	pub enum Error<T> {
		ProofAlreadyExist,
		ProofTooLong,
		ClaimNotExist,
		ClaimIsExpired,
		ClaimIsTransferred,
		NotClaimOwner,
		NewOwnerIsSameAsOldOwner,
		NewOwnerIsNone,
		NewOwnerIsNotExist,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

	// 函数
	#[pallet::call]
	impl<T: Config> Pallet<T> {

		// 创建存证
		#[pallet::weight(T::WeightInfo::create_claim(claim.len() as u32))]
		pub fn create_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保是签名的

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ProofTooLong)?;	// 确保长度不超过最大长度

			ensure!(!Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ProofAlreadyExist); // 确保不存在

			Proofs::<T>::insert(&bounded_claim, (sender.clone(), frame_system::Pallet::<T>::block_number())); 	// 插入数据

			Self::deposit_event(Event::ClaimCreated(sender, claim));	// 发送事件

			Ok(().into())	// 返回
		}

		// 撤销存证
		#[pallet::weight(T::WeightInfo::revoke_claim(claim.len() as u32))]
		pub fn revoke_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保是签名的

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ProofTooLong)?;	// 确保长度不超过最大长度

			ensure!(Proofs::<T>::contains_key(&bounded_claim), Error::<T>::ClaimNotExist);	// 确保存证没有被撤销

			let (owner, _) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;	// 确保存在

			ensure!(sender == owner, Error::<T>::NotClaimOwner);	// 确保是所有者

			Proofs::<T>::remove(&bounded_claim);	// 删除数据

			Self::deposit_event(Event::ClaimRevoked(sender, claim));	// 发送事件

			Ok(().into())	// 返回
		}

		// 转移存证
		#[pallet::weight(T::WeightInfo::transfer_claim(claim.len() as u32))]
		pub fn transfer_claim(
			origin: OriginFor<T>,
			claim: Vec<u8>,
			dest: T::AccountId,
		) -> DispatchResultWithPostInfo {
			let sender = ensure_signed(origin)?; // 确保是签名的

			let bounded_claim = BoundedVec::<u8, T::MaxClaimLength>::try_from(claim.clone()).map_err(|_| Error::<T>::ProofTooLong)?;	// 确保长度不超过最大长度

			let (owner, block_number) = Proofs::<T>::get(&bounded_claim).ok_or(Error::<T>::ClaimNotExist)?;	// 确保存在

			ensure!(sender == owner, Error::<T>::NotClaimOwner);	// 确保是所有者

			ensure!(sender != dest, Error::<T>::NewOwnerIsSameAsOldOwner);	// 确保新旧所有者不一样

			Proofs::<T>::insert(&bounded_claim, (dest.clone(), block_number));	// 更新数据

			Self::deposit_event(Event::ClaimTransferred(sender, dest, claim));	// 发送事件

			Ok(().into())	// 返回
		}

	}
}


