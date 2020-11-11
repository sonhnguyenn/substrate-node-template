
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch};
use frame_system::ensure_signed;
use sp_std::vec::Vec;

use orml_nft;


/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Trait: frame_system::Trait + orml_nft::Trait {
	/// Because this pallet emits events, it depends on the runtime's definition of an event.
	type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Trait> as NftModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<u32>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where 
		AccountId = <T as frame_system::Trait>::AccountId,
		ClassId = <T as orml_nft::Trait>::ClassId,
		TokenData = <T as orml_nft::Trait>::TokenData,
			 {
			CreateClass(AccountId,ClassId),
			MintTokens(AccountId,AccountId,ClassId,TokenData),
			}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Trait> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Trait> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 0]
		pub fn create_class(origin, metadata: Vec<u8>, data: <T as orml_nft::Trait>::ClassData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			let result: Result<T::ClassId,dispatch::DispatchError> = orml_nft::Module::<T>::create_class(&who,metadata,data);
			
			Self::deposit_event(RawEvent::CreateClass(who,result.unwrap()));
			Ok(())
		}
		
		#[weight = 0]
		pub fn mint(origin, dest: T::AccountId,cid: <T as orml_nft::Trait>::ClassId,metadata: Vec<u8>,num: <T as orml_nft::Trait>::TokenData) -> dispatch::DispatchResult {
			let who = ensure_signed(origin)?;
			
			let result: Result<T::TokenId,dispatch::DispatchError> = orml_nft::Module::<T>::mint(&dest,cid,metadata.clone(),num.clone());
			
			Self::deposit_event(RawEvent::MintTokens(who,dest,cid,num));
			Ok(())
		}
		
	}
}
