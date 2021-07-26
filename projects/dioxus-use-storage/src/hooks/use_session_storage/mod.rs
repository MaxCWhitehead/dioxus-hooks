mod display;
mod iter;

use super::*;
use log::warn;

/// effect handler
#[allow(dead_code)]
pub struct UseSessionStorage {
    data: Rc<RefCell<UseLocalSessionData>>,
    listen_storage: Option<EventListener>,
}

#[allow(dead_code)]
struct UseLocalSessionData {
    storage: Option<Storage>,
    last_event: Option<StorageEvent>,
}

impl UseSessionStorage {
    /// builder of `UseCursor`
    pub(crate) fn new(cx: &ScopeState) -> Option<Self> {
        let window = window()?;
        let storage = window.local_storage().ok()??;
        let data = Rc::new(RefCell::new(UseLocalSessionData { storage: Some(storage), last_event: None }));
        let listen_storage = Self::on_storage(cx, &window, &data);
        Some(Self { data, listen_storage: Some(listen_storage) })
    }
    #[inline]
    pub(crate) fn new_ssr(cx: &ScopeState) -> Self {
        #[cfg(debug_assertions)]
        {
            warn!("Window Storage Listener Initializing failed at {}!", cx.scope_id().0);
        }
        Self::default()
    }
    fn on_storage(cx: &ScopeState, window: &Window, data: &Rc<RefCell<UseLocalSessionData>>) -> EventListener {
        #[cfg(debug_assertions)]
        {
            info!("Window Storage Listener Initialized at {}!", cx.scope_id().0);
        }
        let setter = data.clone();
        let regenerate = cx.schedule_update();
        EventListener::new(window, "storage", move |e| {
            let e: StorageEvent = e.clone().unchecked_into();
            // FIXME: find which storage changed
            // e.storage_area();
            let mut setter = setter.borrow_mut();
            setter.last_event = Some(e);
            regenerate()
        })
    }
}

impl UseSessionStorage {
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.borrow().storage.as_ref()?.get_item(key).ok()?
    }
    ///
    #[inline]
    pub fn get_index(&self, index: usize) -> Option<String> {
        self.data.borrow().storage.as_ref()?.key(index as _).ok()?
    }
    /// Getter for the screenX field of this object.
    #[inline]
    pub fn insert(&self, key: &str, value: &str) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.set_item(key, value).is_ok(),
        }
    }
    ///
    #[inline]
    pub fn remove(&self, key: &str) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.remove_item(key).is_ok(),
        }
    }
    ///
    #[inline]
    pub fn len(&self) -> usize {
        match &self.data.borrow().storage {
            None => 0,
            Some(s) => s.length().unwrap_or(0) as _,
        }
    }
    ///
    #[inline]
    pub fn clear(&self) -> bool {
        match &self.data.borrow().storage {
            None => false,
            Some(s) => s.clear().is_ok(),
        }
    }
}
