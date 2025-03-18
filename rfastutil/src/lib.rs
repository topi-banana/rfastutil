use jni::{
    objects::{JClass, JObject, JObjectArray, JString},
    sys::jlong,
    JNIEnv,
};
/*
#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_HelloWorld_hello<'local>(
    // Notice that this `env` argument is mutable. Any `JNIEnv` API that may
    // allocate new object references will take a mutable reference to the
    // environment.
    mut env: JNIEnv<'local>,
    // this is the class that owns our static method. Not going to be used, but
    // still needs to have an argument slot
    _class: JClass<'local>,
    input: JString<'local>,
) -> JString<'local> {
    // First, we have to get the string out of java. Check out the `strings`
    // module for more info on how this works.
    let input: String = env
        .get_string(&input)
        .expect("Couldn't get java string!")
        .into();

    // Then we have to create a new java string to return. Again, more info
    // in the `strings` module.
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");
    output
}
*/

fn murmur_hash3(mut x: i64) -> i64 {
    x ^= x >> 33;
    x = x.wrapping_mul(-49064778989728563_i64);
    x ^= x >> 33;
    x = x.wrapping_mul(-4265267296055464877_i64);
    x ^= x >> 33;
    x
}



#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_longs_Long2ObjectHashMap_get<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    arr: JObjectArray<'local>,
    key: jlong,
) -> JObject<'local> {
    let len = env.get_array_length(&arr).unwrap();
    let mask = len - 1;
    let mut pos = (murmur_hash3(key) & mask as i64) as usize;

    loop {
        // Get the element at the current position.
        let element = env.get_object_array_element(&arr, pos as i32).unwrap();

        // Check if the element is null.
        if env.is_same_object(&element, &JObject::null()).unwrap() {
            return JObject::null(); // Return null if the element is null.
        }

        // Get the key and value from the element.
        let key_field = env.get_field(&element, "key", "J").unwrap().j().unwrap();

        // Compare the key with the given key.
        if key_field == key {
            let value_field = env.get_field(&element, "value", "Ljava/lang/Object;").unwrap().l().unwrap();
            return value_field; // Return the value if the key matches.
        }

        pos = (pos + 1) & mask as usize; // Move to the next position.
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_longs_Long2ObjectHashMap_put<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    arr: JObjectArray<'local>,
    key: jlong,
    value: JObject<'local>,
) -> JObject<'local> {

    let len = env.get_array_length(&arr).unwrap();
    let mask = len - 1;
    let mut pos = (murmur_hash3(key) & mask as i64) as usize;

    loop {
        // Get the element at the current position.
        let element = env.get_object_array_element(&arr, pos as i32).unwrap();

        // Check if the element is null.
        if env.is_same_object(&element, &JObject::null()).unwrap() {
            // Create a new entry object.
            let entry_class = env.find_class("rfastutil/longs/Long2ObjectHashMap$Entry").unwrap();
            let new_entry = env.new_object(&entry_class, "(JLjava/lang/Object;)V", &[key.into(), (&value).into()]).unwrap();

            // Set the new entry to the array.
            env.set_object_array_element(&arr, pos as i32, new_entry).unwrap();

            return JObject::null(); // Return null for new entry
        }

        // Get the key and value from the element.
        let key_field = env.get_field(&element, "key", "J").unwrap().j().unwrap();

        // Compare the key with the given key.
        if key_field == key {
            let value_field = env.get_field(&element, "value", "Ljava/lang/Object;").unwrap().l().unwrap();
            env.set_field(&element, "value", "Ljava/lang/Object;", (&value).into()).unwrap();
            return value_field; // Return the old value if the key matches.
        }

        pos = (pos + 1) & mask as usize; // Move to the next position.
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_longs_Long2ObjectHashMap_remove<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    arr: JObjectArray<'local>,
    key: jlong,
) -> JObject<'local> {

    let len = env.get_array_length(&arr).unwrap();
    let mask = len - 1;
    let mut pos = (murmur_hash3(key) & mask as i64) as usize;

    loop {
        // Get the element at the current position.
        let element = env.get_object_array_element(&arr, pos as i32).unwrap();

        // Check if the element is null.
        if env.is_same_object(&element, &JObject::null()).unwrap() {
            return JObject::null(); // Return null if the element is null.
        }

        // Get the key and value from the element.
        let key_field = env.get_field(&element, "key", "J").unwrap().j().unwrap();

        // Compare the key with the given key.
        if key_field == key {
            let value_field = env.get_field(&element, "value", "Ljava/lang/Object;").unwrap().l().unwrap();
            // Set the element to null to remove it.
            env.set_object_array_element(&arr, pos as i32, JObject::null()).unwrap();
            return value_field; // Return the removed value if the key matches.
        }

        pos = (pos + 1) & mask as usize; // Move to the next position.
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_longs_Long2ObjectHashMap_containsKey<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    arr: JObjectArray<'local>,
    key: jlong,
) -> bool {

    let len = env.get_array_length(&arr).unwrap();
    let mask = len - 1;
    let mut pos = (murmur_hash3(key) & mask as i64) as usize;

    loop {
        // Get the element at the current position.
        let element = env.get_object_array_element(&arr, pos as i32).unwrap();

        // Check if the element is null.
        if env.is_same_object(&element, &JObject::null()).unwrap() {
            return false; // Return false if the element is null.
        }

        // Get the key and value from the element.
        let key_field = env.get_field(&element, "key", "J").unwrap().j().unwrap();

        // Compare the key with the given key.
        if key_field == key {
            return true; // Return true if the key matches.
        }

        pos = (pos + 1) & mask as usize; // Move to the next position.
    }
}

#[unsafe(no_mangle)]
pub extern "system" fn Java_rfastutil_longs_Long2ObjectHashMap_clear<'local>(
    env: JNIEnv<'local>,
    _class: JClass,
    arr: JObjectArray<'local>,
) {
    let len = env.get_array_length(&arr).unwrap();

    for i in 0..len {
        env.set_object_array_element(&arr, i as i32, JObject::null()).unwrap();
    }
}

