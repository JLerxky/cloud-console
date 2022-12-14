use console::cli::{
    emergency_brake, set_block_interval, set_quota_limit, update_admin, update_validators,
};
use jni::objects::{JClass, JString};
use jni::sys::{jobject, jstring};
use jni::JNIEnv;

#[no_mangle]
pub extern "system" fn Java_Console_updateAdmin(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    account_addr: JString,
    new_admin: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let account_addr: String = env
        .get_string(account_addr)
        .expect("Couldn't get java string!")
        .into();
    let new_admin: String = env
        .get_string(new_admin)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(update_admin(controller_addr, crypto_addr, account_addr, new_admin).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_Console_setBlockInterval(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    account_addr: JString,
    block_interval: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let account_addr: String = env
        .get_string(account_addr)
        .expect("Couldn't get java string!")
        .into();
    let block_interval: String = env
        .get_string(block_interval)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(
        set_block_interval(controller_addr, crypto_addr, account_addr, block_interval).to_json(),
    )
    .expect("Couldn't create java string!")
    .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_Console_updateValidators(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    account_addr: JString,
    validators_jobject: jobject,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let account_addr: String = env
        .get_string(account_addr)
        .expect("Couldn't get java string!")
        .into();
    let validators_len = env.get_array_length(validators_jobject).unwrap();
    let mut validators: Vec<String> = Vec::with_capacity(validators_len as usize);
    for i in 0..validators_len {
        let j_object = env.get_object_array_element(validators_jobject, i).unwrap();
        let validator = env
            .get_string(JString::from(j_object))
            .expect("Couldn't get java string!")
            .into();
        validators.push(validator);
    }

    env.new_string(
        update_validators(controller_addr, crypto_addr, account_addr, validators).to_json(),
    )
    .expect("Couldn't create java string!")
    .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_Console_emergencyBrake(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    account_addr: JString,
    switch: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let account_addr: String = env
        .get_string(account_addr)
        .expect("Couldn't get java string!")
        .into();
    let switch: String = env
        .get_string(switch)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(emergency_brake(controller_addr, crypto_addr, account_addr, switch).to_json())
        .expect("Couldn't create java string!")
        .into_raw()
}

#[no_mangle]
pub extern "system" fn Java_Console_setQuotaLimit(
    env: JNIEnv,
    _class: JClass,
    controller_addr: JString,
    crypto_addr: JString,
    account_addr: JString,
    quota_limit: JString,
) -> jstring {
    let controller_addr: String = env
        .get_string(controller_addr)
        .expect("Couldn't get java string!")
        .into();
    let crypto_addr: String = env
        .get_string(crypto_addr)
        .expect("Couldn't get java string!")
        .into();
    let account_addr: String = env
        .get_string(account_addr)
        .expect("Couldn't get java string!")
        .into();
    let quota_limit: String = env
        .get_string(quota_limit)
        .expect("Couldn't get java string!")
        .into();

    env.new_string(
        set_quota_limit(controller_addr, crypto_addr, account_addr, quota_limit).to_json(),
    )
    .expect("Couldn't create java string!")
    .into_raw()
}
