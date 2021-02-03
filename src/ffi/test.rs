use super::*;

//
// Helper methods for many tests
//

/// Create and return a metafits context based on a test metafits file. Used in many tests in the module.
///
///
/// # Arguments
///
/// * None
///
///
/// # Returns
///
/// * a raw pointer to an instantiated MetafitsContext for the test metafits and gpubox file
///
fn get_test_metafits_context() -> *mut MetafitsContext {
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    let metafits_file =
        CString::new("test_files/1101503312_1_timestep/1101503312.metafits").unwrap();
    let metafits_file_ptr = metafits_file.as_ptr();

    unsafe {
        // Create a MetafitsContext
        let mut metafits_context_ptr: *mut MetafitsContext = std::ptr::null_mut();
        let retval = mwalib_metafits_context_new(
            metafits_file_ptr,
            &mut metafits_context_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value of mwalib_metafits_context_new
        assert_eq!(retval, 0, "mwalib_metafits_context_new failure");

        // Check we got valid MetafitsContext pointer
        let context_ptr = metafits_context_ptr.as_mut();
        assert!(context_ptr.is_some());

        context_ptr.unwrap()
    }
}

/// Create and return a metafits context based on a test metafits and gpubox file. Used in many tests in the module.
///
///
/// # Arguments
///
/// * None
///
///
/// # Returns
///
/// * a raw pointer to an instantiated CorrelatorContext for the test metafits and gpubox file
///
fn get_test_correlator_context() -> *mut CorrelatorContext {
    // This tests for a valid correlator context
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    let metafits_file =
        CString::new("test_files/1101503312_1_timestep/1101503312.metafits").unwrap();
    let metafits_file_ptr = metafits_file.as_ptr();

    let gpubox_file =
        CString::new("test_files/1101503312_1_timestep/1101503312_20141201210818_gpubox01_00.fits")
            .unwrap();
    let mut gpubox_files: Vec<*const c_char> = Vec::new();
    gpubox_files.push(gpubox_file.as_ptr());
    let gpubox_files_ptr = gpubox_files.as_ptr() as *mut *const c_char;

    unsafe {
        // Create a CorrelatorContext
        let mut correlator_context_ptr: *mut CorrelatorContext = std::ptr::null_mut();
        let retval = mwalib_correlator_context_new(
            metafits_file_ptr,
            gpubox_files_ptr,
            1,
            &mut correlator_context_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value of mwalib_correlator_context_new
        assert_eq!(retval, 0, "mwalib_correlator_context_new failure");

        // Check we got valid MetafitsContext pointer
        let context_ptr = correlator_context_ptr.as_mut();
        assert!(context_ptr.is_some());

        context_ptr.unwrap()
    }
}

/// Reconstructs a Vec<T> from FFI using a pointer to a rust-allocated array of *mut T.
///
///
/// # Arguments
///
/// * `ptr` - raw pointer pointing to an array of T
///
/// * 'len' - number of elements in the array
///
///
/// # Returns
///
/// * Array of T expressed as Vec<T>
///
fn ffi_boxed_slice_to_array<T>(ptr: *mut T, len: usize) -> Vec<T> {
    unsafe {
        let vec: Vec<T> = Vec::from_raw_parts(ptr, len, len);
        vec
    }
}

//
// Simple test of the error message helper
//
#[test]
fn test_set_error_message() {
    let buffer = CString::new("HELLO WORLD").unwrap();
    let buffer_ptr = buffer.as_ptr() as *mut u8;

    set_error_message("hello world", buffer_ptr, 12);

    assert_eq!(buffer, CString::new("hello world").unwrap());
}

//
// Metafits context Tests
//
#[test]
fn test_mwalib_metafits_context_new_valid() {
    // This tests for a valid metafitscontext
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    let metafits_file =
        CString::new("test_files/1101503312_1_timestep/1101503312.metafits").unwrap();
    let metafits_file_ptr = metafits_file.as_ptr();

    unsafe {
        // Create a MetafitsContext
        let mut metafits_context_ptr: *mut MetafitsContext = std::ptr::null_mut();
        let retval = mwalib_metafits_context_new(
            metafits_file_ptr,
            &mut metafits_context_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value of mwalib_metafits_context_new
        assert_eq!(retval, 0, "mwalib_metafits_context_new failure");

        // Check we got valid MetafitsContext pointer
        let context_ptr = metafits_context_ptr.as_mut();
        assert!(context_ptr.is_some());
    }
}

//
// CorrelatorContext Tests
//
#[test]
fn test_mwalib_correlator_context_new_valid() {
    // This tests for a valid correlator context
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    let metafits_file =
        CString::new("test_files/1101503312_1_timestep/1101503312.metafits").unwrap();
    let metafits_file_ptr = metafits_file.as_ptr();

    let gpubox_file =
        CString::new("test_files/1101503312_1_timestep/1101503312_20141201210818_gpubox01_00.fits")
            .unwrap();
    let mut gpubox_files: Vec<*const c_char> = Vec::new();
    gpubox_files.push(gpubox_file.as_ptr());
    let gpubox_files_ptr = gpubox_files.as_ptr() as *mut *const c_char;

    unsafe {
        // Create a CorrelatorContext
        let mut correlator_context_ptr: *mut CorrelatorContext = std::ptr::null_mut();
        let retval = mwalib_correlator_context_new(
            metafits_file_ptr,
            gpubox_files_ptr,
            1,
            &mut correlator_context_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value of mwalib_correlator_context_new
        assert_eq!(retval, 0, "mwalib_correlator_context_new failure");

        // Check we got valid MetafitsContext pointer
        let context_ptr = correlator_context_ptr.as_mut();
        assert!(context_ptr.is_some());
    }
}

//
// Metafits Metadata Tests
//
#[test]
fn test_mwalib_metafits_metadata_get_from_metafits_context_valid() {
    // This tests for a valid metafits context and metadata returned
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;
    // Create a MetafitsContext
    let metafits_context_ptr: *mut MetafitsContext = get_test_metafits_context();
    unsafe {
        // Check we got valid MetafitsContext pointer
        let context_ptr = metafits_context_ptr.as_mut();
        assert!(context_ptr.is_some());

        // Populate a mwalibMetafitsMetadata struct
        let mut metafits_metadata_ptr: &mut *mut mwalibMetafitsMetadata = &mut std::ptr::null_mut();
        let retval = mwalib_metafits_metadata_get(
            metafits_context_ptr,
            std::ptr::null_mut(),
            &mut metafits_metadata_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value
        assert_eq!(
            retval, 0,
            "mwalib_metafits_metadata_get did not return success"
        );

        // Get the mwalibMetadata struct from the pointer
        let metafits_metadata = Box::from_raw(*metafits_metadata_ptr);

        // We should get a valid obsid and no error message
        assert_eq!(metafits_metadata.obsid, 1_101_503_312);
    }
}

#[test]
fn test_mwalib_metafits_metadata_get_null_contexts() {
    // This tests for a null context passed to mwalib_metafits_metadata_get
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut metafits_metadata_ptr: &mut *mut mwalibMetafitsMetadata = &mut std::ptr::null_mut();

        let context_ptr = std::ptr::null_mut();
        let metafits_ptr = std::ptr::null_mut();
        let ret_val = mwalib_metafits_metadata_get(
            context_ptr,
            metafits_ptr,
            &mut metafits_metadata_ptr,
            error_message_ptr,
            error_len,
        );

        // We should get a non-zero return code
        assert_ne!(ret_val, 0);
    }
}

#[test]
fn test_mwalib_metafits_metadata_get_from_correlator_context_valid() {
    // This tests for a valid metafits metadata returned given a correlator context
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        // Create a CorrelatorContext
        let correlator_context_ptr: *mut CorrelatorContext = get_test_correlator_context();

        // Check we got valid MetafitsContext pointer
        let context_ptr = correlator_context_ptr.as_mut();
        assert!(context_ptr.is_some());

        // Populate a mwalibMetafitsMetadata struct
        let mut metafits_metadata_ptr: &mut *mut mwalibMetafitsMetadata = &mut std::ptr::null_mut();
        let retval = mwalib_metafits_metadata_get(
            std::ptr::null_mut(),
            correlator_context_ptr,
            &mut metafits_metadata_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value
        assert_eq!(
            retval, 0,
            "mwalib_metafits_metadata_get did not return success"
        );

        // Get the mwalibMetadata struct from the pointer
        let metafits_metadata = Box::from_raw(*metafits_metadata_ptr);

        // We should get a valid obsid and no error message
        assert_eq!(metafits_metadata.obsid, 1_101_503_312);
    }
}

#[test]
fn test_mwalib_correlator_metadata_get_valid() {
    // This tests for a valid correlator metadata struct being instatiated
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *mut c_char;
    let error_len: size_t = 60;

    unsafe {
        // Create a CorrelatorContext
        let correlator_context_ptr: *mut CorrelatorContext = get_test_correlator_context();

        // Check we got valid MetafitsContext pointer
        let context_ptr = correlator_context_ptr.as_mut();
        assert!(context_ptr.is_some());

        // Populate a mwalibCorrelatorMetadata struct
        let mut correlator_metadata_ptr: &mut *mut mwalibCorrelatorMetadata =
            &mut std::ptr::null_mut();
        let retval = mwalib_correlator_metadata_get(
            correlator_context_ptr,
            &mut correlator_metadata_ptr,
            error_message_ptr,
            error_len,
        );

        // Check return value
        assert_eq!(
            retval, 0,
            "mwalib_correlator_metadata_get did not return success"
        );

        // Get the mwalibMetadata struct from the pointer
        let correlator_metadata = Box::from_raw(*correlator_metadata_ptr);

        // We should get a valid timestep and no error message
        assert_eq!(correlator_metadata.integration_time_milliseconds, 2000);
    }
}

#[test]
fn test_mwalib_correlator_metadata_get_null_context() {
    // This tests for passing a null context to the mwalib_correlator_metadata_get() method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut correlator_metadata_ptr: &mut *mut mwalibCorrelatorMetadata =
            &mut std::ptr::null_mut();

        let context_ptr = std::ptr::null_mut();
        let ret_val = mwalib_correlator_metadata_get(
            context_ptr,
            &mut correlator_metadata_ptr,
            error_message_ptr,
            error_len,
        );

        // We should get a non-zero return code
        assert_ne!(ret_val, 0);
    }
}

#[test]
fn test_mwalib_antennas_get_from_metafits_context_valid() {
    // This test populates antennas given a metafits context
    let index = 2; // valid  should be Tile013

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_metafits_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibAntenna = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_antennas_get(
            context,
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_antennas_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibAntenna> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 128, "Array length is not correct");
        assert_eq!(item[index].tile_id, 13);
    }
}

#[test]
fn test_mwalib_antennas_get_from_correlator_context_valid() {
    // This test populates antennas given a correlator context
    let index = 2; // valid  should be Tile013
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibAntenna = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_antennas_get(
            std::ptr::null_mut(),
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_antennas_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibAntenna> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 128, "Array length is not correct");
        assert_eq!(item[index].tile_id, 13);
    }
}

#[test]
fn test_mwalib_antennas_get_null_contexts() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibAntenna = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_antennas_get(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_antennas_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}

// Baselines
#[test]
fn test_mwalib_correlator_baselines_get_valid() {
    // This test populates baselines given a correlator context
    let index = 2; // valid  should be baseline (0,2)

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibBaseline = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_correlator_baselines_get(
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_baselines_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibBaseline> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 8256, "Array length is not correct");
        assert_eq!(item[index].antenna1_index, 0);
        assert_eq!(item[index].antenna2_index, 2);
    }
}

#[test]
fn test_mwalib_correlator_baselines_get_null_context() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibBaseline = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_correlator_baselines_get(
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_correlator_baselines_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}

// Coarse Channels
#[test]
fn test_mwalib_correlator_coarse_channels_get_valid() {
    // This test populates coarse_channels given a correlator context
    let index = 0; // valid  should be receiver channel 109

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibCoarseChannel = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_correlator_coarse_channels_get(
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(
            retval, 0,
            "mwalib_correlator_coarse_channels_get did not return success"
        );

        // reconstitute into a vector
        let item: Vec<mwalibCoarseChannel> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 1, "Array length is not correct");
        assert_eq!(item[index].receiver_channel_number, 109);
    }
}

#[test]
fn test_mwalib_correlator_coarse_channels_get_null_context() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibCoarseChannel = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_correlator_coarse_channels_get(
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_correlator_coarse_channels_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}

// RF Input
#[test]
fn test_mwalib_rfinputs_get_from_metafits_context_valid() {
    // This test populates rfinputs given a metafits context
    let index = 2; // valid  should be Tile012(X)

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_metafits_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibRFInput = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_rfinputs_get(
            context,
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_rfinputs_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibRFInput> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 256, "Array length is not correct");

        assert_eq!(item[index].antenna, 1);

        assert_eq!(
            CString::from_raw(item[index].tile_name),
            CString::new("Tile012").unwrap()
        );

        assert_eq!(
            CString::from_raw(item[index].pol),
            CString::new("X").unwrap()
        );
    }
}

#[test]
fn test_mwalib_rfinputs_get_from_correlator_context_valid() {
    // This test populates rfinputs given a correlator context
    let index = 2; // valid  should be Tile012(X)

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibRFInput = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_rfinputs_get(
            std::ptr::null_mut(),
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_rfinputs_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibRFInput> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 256, "Array length is not correct");

        assert_eq!(item[index].antenna, 1);

        assert_eq!(
            CString::from_raw(item[index].tile_name),
            CString::new("Tile012").unwrap()
        );

        assert_eq!(
            CString::from_raw(item[index].pol),
            CString::new("X").unwrap()
        );
    }
}

#[test]
fn test_mwalib_rfinputs_get_null_contexts() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibRFInput = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_rfinputs_get(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_rfinputs_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}

// Timesteps
#[test]
fn test_mwalib_correlator_timesteps_get_valid() {
    // This test populates timesteps given a correlator context
    let index = 0; // valid  should be timestep at unix_time 1417468096.0;

    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibTimeStep = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_correlator_timesteps_get(
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(retval, 0, "mwalib_timesteps_get did not return success");

        // reconstitute into a vector
        let item: Vec<mwalibTimeStep> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 1, "Array length is not correct");
        assert_eq!(item[index].unix_time_ms, 1_417_468_096_000);
    }
}

#[test]
fn test_mwalib_correlator_timesteps_get_null_context() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibTimeStep = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_correlator_timesteps_get(
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_correlator_timesteps_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}

// Visibility Pols
#[test]
fn test_mwalib_correlator_visibility_pols_get_valid() {
    // This test populates visibility_pols given a correlator context
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let context = get_test_correlator_context();

        // Check we got a context object
        let context_ptr = context.as_mut();
        assert!(context_ptr.is_some());

        let mut array_ptr: &mut *mut mwalibVisibilityPol = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;

        let retval = mwalib_correlator_visibility_pols_get(
            context,
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // check ret val is ok
        assert_eq!(
            retval, 0,
            "mwalib_correlator_visibility_pols_get did not return success"
        );

        // reconstitute into a vector
        let item: Vec<mwalibVisibilityPol> = ffi_boxed_slice_to_array(*array_ptr, array_len);

        // We should get a valid, populated array
        assert_eq!(array_len, 4, "Array length is not correct");
        assert_eq!(
            CString::from_raw(item[0].polarisation),
            CString::new("XX").unwrap()
        );
        assert_eq!(
            CString::from_raw(item[1].polarisation),
            CString::new("XY").unwrap()
        );
        assert_eq!(
            CString::from_raw(item[2].polarisation),
            CString::new("YX").unwrap()
        );
        assert_eq!(
            CString::from_raw(item[3].polarisation),
            CString::new("YY").unwrap()
        );
    }
}

#[test]
fn test_mwalib_correlator_visibilitypols_get_null_context() {
    // This tests for a null context passed into the _get method
    let error_message =
        CString::new("                                                            ").unwrap();
    let error_message_ptr = error_message.as_ptr() as *const c_char;
    let error_len: size_t = 60;

    unsafe {
        let mut array_ptr: &mut *mut mwalibVisibilityPol = &mut std::ptr::null_mut();
        let mut array_len: usize = 0;
        let retval = mwalib_correlator_visibility_pols_get(
            std::ptr::null_mut(),
            &mut array_ptr,
            &mut array_len,
            error_message_ptr,
            error_len,
        );

        // We should get a null pointer, non-zero retval and an error message
        assert_ne!(retval, 0);
        assert!(array_ptr.is_null());
        let expected_error: &str = &"mwalib_correlator_visibility_pols_get() ERROR:";
        assert_eq!(
            error_message.into_string().unwrap()[0..expected_error.len()],
            *expected_error
        );
    }
}
