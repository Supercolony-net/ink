use core::marker::PhantomData;
use ink_primitives::{
    Key,
    KeyPtr,
};
use ink_storage::traits::{
    PackedAllocate,
    PackedLayout,
    SpreadAllocate,
    SpreadLayout,
};
use scale::{
    Decode,
    Encode,
};

#[derive(Debug)]
pub struct Initialized;
#[derive(Debug)]
pub struct NotInitialized;

#[derive(Debug, Decode, Encode)]
#[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
pub struct Upgradable<T: PackedLayout, InitializationStatus = Initialized> {
    inner: T,
    status: PhantomData<fn() -> InitializationStatus>,
}

impl<T: PackedLayout, State> Upgradable<T, State> {
    pub fn new(inner: T) -> Self {
        Upgradable {
            inner,
            status: Default::default(),
        }
    }
}

impl<T: PackedLayout> SpreadLayout for Upgradable<T, Initialized> {
    const FOOTPRINT: u64 = T::FOOTPRINT;
    const REQUIRES_DEEP_CLEAN_UP: bool = T::REQUIRES_DEEP_CLEAN_UP;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        Upgradable::new(T::pull_spread(ptr))
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        T::push_spread(&self.inner, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        T::clear_spread(&self.inner, ptr)
    }
}

impl<T: PackedLayout + SpreadAllocate> SpreadLayout for Upgradable<T, NotInitialized> {
    const FOOTPRINT: u64 = <T as SpreadLayout>::FOOTPRINT;
    const REQUIRES_DEEP_CLEAN_UP: bool = <T as SpreadLayout>::REQUIRES_DEEP_CLEAN_UP;

    fn pull_spread(ptr: &mut KeyPtr) -> Self {
        if ink_env::get_contract_storage::<T>(ptr.key())
            .expect("could not properly decode storage entry")
            .is_none()
        {
            <Self as SpreadAllocate>::allocate_spread(ptr)
        } else {
            Upgradable::new(<T as SpreadLayout>::pull_spread(ptr))
        }
    }

    fn push_spread(&self, ptr: &mut KeyPtr) {
        <T as SpreadLayout>::push_spread(&self.inner, ptr)
    }

    fn clear_spread(&self, ptr: &mut KeyPtr) {
        <T as SpreadLayout>::clear_spread(&self.inner, ptr)
    }
}

impl<T: PackedLayout> PackedLayout for Upgradable<T, Initialized> {
    fn pull_packed(&mut self, at: &Key) {
        <T as PackedLayout>::pull_packed(&mut self.inner, at)
    }

    fn push_packed(&self, at: &Key) {
        <T as PackedLayout>::push_packed(&self.inner, at)
    }

    fn clear_packed(&self, at: &Key) {
        <T as PackedLayout>::clear_packed(&self.inner, at)
    }
}

impl<T: PackedLayout + SpreadAllocate> PackedLayout for Upgradable<T, NotInitialized> {
    fn pull_packed(&mut self, at: &Key) {
        <T as PackedLayout>::pull_packed(&mut self.inner, at)
    }

    fn push_packed(&self, at: &Key) {
        <T as PackedLayout>::push_packed(&self.inner, at)
    }

    fn clear_packed(&self, at: &Key) {
        <T as PackedLayout>::clear_packed(&self.inner, at)
    }
}

impl<T: SpreadAllocate + PackedLayout> SpreadAllocate for Upgradable<T, Initialized> {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        Upgradable::new(<T as SpreadAllocate>::allocate_spread(ptr))
    }
}

impl<T: SpreadAllocate + PackedLayout> SpreadAllocate for Upgradable<T, NotInitialized> {
    fn allocate_spread(ptr: &mut KeyPtr) -> Self {
        Upgradable::new(<T as SpreadAllocate>::allocate_spread(ptr))
    }
}

impl<T: PackedAllocate> PackedAllocate for Upgradable<T, Initialized> {
    fn allocate_packed(&mut self, at: &Key) {
        <T as PackedAllocate>::allocate_packed(&mut self.inner, at)
    }
}

impl<T: PackedAllocate> PackedAllocate for Upgradable<T, NotInitialized> {
    fn allocate_packed(&mut self, at: &Key) {
        <T as PackedAllocate>::allocate_packed(&mut self.inner, at)
    }
}

impl<T: PackedLayout, State> core::ops::Deref for Upgradable<T, State> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: PackedLayout, State> core::ops::DerefMut for Upgradable<T, State> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: PackedLayout, State> AsRef<T> for Upgradable<T, State> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<T: PackedLayout, State> AsMut<T> for Upgradable<T, State> {
    #[inline]
    fn as_mut(&mut self) -> &mut T {
        &mut self.inner
    }
}

impl<T: PackedLayout + Default, State> Default for Upgradable<T, State> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

#[cfg(feature = "std")]
const _: () = {
    use ink_metadata::layout::Layout;
    use ink_storage::traits::StorageLayout;

    impl<T, State> StorageLayout for Upgradable<T, State>
    where
        T: PackedLayout + StorageLayout + scale_info::TypeInfo + 'static,
    {
        fn layout(key_ptr: &mut KeyPtr) -> Layout {
            <T as StorageLayout>::layout(key_ptr)
        }
    }
};
