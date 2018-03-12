extern crate error_chain;

#[macro_use]
extern crate failure;

extern crate hermes;

extern crate hermes_ffi;

extern crate hermes_mqtt;

extern crate libc;

#[macro_use]
extern crate ffi_utils;

use hermes::ResultExt as HResultExt;

use failure::ResultExt;

use hermes_ffi::CProtocolHandler;

use ffi_utils::{C_RESULT, RawPointerConverter};

#[no_mangle]
pub extern "C" fn hermes_protocol_handler_new_mqtt(handler: *mut *const CProtocolHandler, broker_address: *const libc::c_char) -> C_RESULT {
    fn new_mqtt_handler(handler: *mut *const CProtocolHandler, broker_address: *const libc::c_char) -> Result<(), failure::Error>{
        let address = create_rust_string_from!(broker_address);
        let cph = CProtocolHandler::new(hermes_mqtt::MqttHermesProtocolHandler::new(&address).map_err(|e| format_err!("Could not create hermes MQTT handler : {:?}", e))?);
        let ptr = CProtocolHandler::into_raw_pointer(cph);
        unsafe {
            *handler = ptr;
        }
        Ok(())
    }
    wrap!(new_mqtt_handler(handler, broker_address).compat().chain_err(||"could not create handler"))
}


#[no_mangle]
pub extern "C" fn hermes_destroy_mqtt_protocol_handler(handler: *mut CProtocolHandler) -> C_RESULT {
    fn destroy_mqtt_handler(handler: *mut CProtocolHandler) -> hermes::Result<()>{
        let handler = unsafe  { CProtocolHandler::from_raw_pointer(handler) }.compat().chain_err(||"could not convert from raw pointer")?;
        handler.destroy::<hermes_mqtt::MqttHermesProtocolHandler>();
        Ok(())
    }
    wrap!(destroy_mqtt_handler(handler))
}