// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/*!
This module exists purely for other languages to interface with mwalib.
 */

use crate::*;
use libc::{c_char, c_float, size_t};
use std::ffi::*;
use std::mem;
use std::slice;

#[cfg(test)]
mod test;

/// Generic helper function for all FFI modules to take an already allocated C string
/// and update it with an error message. This is used to pass error messages back to C from Rust.
///
/// # Arguments
///
/// * `in_message` - A Rust string holing the error message you want to pass back to C
///
/// * `error_buffer_ptr` - Pointer to a char* buffer which has already been allocated, for storing the error message.
///
/// * `error_buffer_len` - Length of char* buffer allocated by caller in C.
///
///
/// # Returns
///
/// * Nothing
///
///
/// # Safety
/// It is up to the caller to:
/// - Allocate `error_buffer_len` bytes as a `char*` on the heap
/// - Free `error_buffer_ptr` once finished with the buffer
///
fn set_error_message(in_message: &str, error_buffer_ptr: *mut u8, error_buffer_len: size_t) {
    // Don't do anything if the pointer is null.
    if error_buffer_ptr.is_null() {
        return;
    }
    // Check that error buffer, minus 1 for nul terminator is still >=1
    if error_buffer_len as i32 - 1 < 1 {
        return;
    }
    // Trim it to error_buffer_len - 1 (must include room for null terminator)
    let in_buffer_len = in_message.len();
    let message = if in_buffer_len > error_buffer_len {
        &in_message[..error_buffer_len - 1]
    } else {
        in_message
    };

    // Convert to C string- panic if it can't.
    let error_message = CString::new(message).unwrap();

    // Add null terminator
    let error_message_bytes = error_message.as_bytes();

    unsafe {
        // Reconstruct a string to write into
        let error_message_slice = slice::from_raw_parts_mut(error_buffer_ptr, error_buffer_len);

        // Copy in the bytes
        error_message_slice[..error_message_bytes.len()].copy_from_slice(error_message_bytes);
    }
}

/// Free a rust-allocated CString.
///
/// mwalib uses error strings to detail the caller with anything that went
/// wrong. Non-rust languages cannot deallocate these strings; so, call this
/// function with the pointer to do that.
///
/// # Arguments
///
/// * `rust_cstring` - pointer to a `char*` of a Rust string
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
/// # Safety
/// * rust_cstring must not have already been freed and must point to a Rust string.
#[no_mangle]
pub unsafe extern "C" fn mwalib_free_rust_cstring(rust_cstring: *mut c_char) -> i32 {
    // Don't do anything if the pointer is null.
    if rust_cstring.is_null() {
        return 0;
    }
    CString::from_raw(rust_cstring);

    // return success
    0
}

/// Boxes for FFI a rust-allocated vector of T.
///
///
/// # Arguments
///
/// * `v` - Rust vector of T's
///
///
/// # Returns
///
/// * a raw pointer to the array of T's
///
fn ffi_array_to_boxed_slice<T>(v: Vec<T>) -> *mut T {
    let mut boxed_slice: Box<[T]> = v.into_boxed_slice();
    let array_ptr: *mut T = boxed_slice.as_mut_ptr();
    let array_ptr_len: usize = boxed_slice.len();
    assert_eq!(boxed_slice.len(), array_ptr_len);

    // Prevent the slice from being destroyed (Leak the memory).
    // This is because we are using our ffi code to free the memory
    mem::forget(boxed_slice);

    array_ptr
}

/// Create and return a pointer to an `MetafitsContext` struct given only a metafits file
///
/// # Arguments
///
/// * `metafits_filename` - pointer to char* buffer containing the full path and filename of a metafits file.
///
/// * `out_metafits_context_ptr` - A Rust-owned populated `MetafitsContext` pointer. Free with `mwalib_metafits_context_free'.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated `char*` buffer for any error messages.
/// * Caller *must* call the `mwalib_metafits_context_free` function to release the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_metafits_context_new(
    metafits_filename: *const c_char,
    out_metafits_context_ptr: &mut *mut MetafitsContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    let m = CStr::from_ptr(metafits_filename)
        .to_str()
        .unwrap()
        .to_string();
    let context = match MetafitsContext::new(&m) {
        Ok(c) => c,
        Err(e) => {
            set_error_message(
                &format!("{}", e),
                error_message as *mut u8,
                error_message_length,
            );
            // Return failure
            return 1;
        }
    };

    *out_metafits_context_ptr = Box::into_raw(Box::new(context));

    // Return success
    0
}

/// Display an `MetafitsContext` struct.
///
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `metafits_context_ptr` must contain an MetafitsContext object already populated via `mwalib_metafits_context_new`
#[no_mangle]
pub unsafe extern "C" fn mwalib_metafits_context_display(
    metafits_context_ptr: *const MetafitsContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if metafits_context_ptr.is_null() {
        set_error_message(
            "mwalib_metafits_context_display() ERROR: null pointer for metafits_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }

    let context = &*metafits_context_ptr;

    println!("{}", context);

    // Return success
    0
}

/// Free a previously-allocated `MetafitsContext` struct (and it's members).
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `MetafitsContext` object
/// * `metafits_context_ptr` must point to a populated `MetafitsContext` object from the `mwalib_metafits_context_new` functions.
/// * `metafits_context_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_metafits_context_free(
    metafits_context_ptr: *mut MetafitsContext,
) -> i32 {
    if metafits_context_ptr.is_null() {
        return 0;
    }

    // Release correlator context if applicable
    Box::from_raw(metafits_context_ptr);

    // Return success
    0
}

/// Create and return a pointer to an `CorrelatorContext` struct based on metafits and gpubox files
///
/// # Arguments
///
/// * `metafits_filename` - pointer to char* buffer containing the full path and filename of a metafits file.
///
/// * `gpubox_filenames` - pointer to array of char* buffers containing the full path and filename of the gpubox FITS files.
///
/// * `gpubox_count` - length of the gpubox char* array.
///
/// * `out_correlator_context_ptr` - A Rust-owned populated `CorrelatorContext` pointer. Free with `mwalib_correlator_context_free`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated `char*` buffer for any error messages.
/// * Caller *must* call function `mwalib_correlator_context_free` to release the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_context_new(
    metafits_filename: *const c_char,
    gpubox_filenames: *mut *const c_char,
    gpubox_count: size_t,
    out_correlator_context_ptr: &mut *mut CorrelatorContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    let m = CStr::from_ptr(metafits_filename)
        .to_str()
        .unwrap()
        .to_string();
    let gpubox_slice = slice::from_raw_parts(gpubox_filenames, gpubox_count);
    let mut gpubox_files = Vec::with_capacity(gpubox_count);
    for g in gpubox_slice {
        let s = CStr::from_ptr(*g).to_str().unwrap();
        gpubox_files.push(s.to_string())
    }
    let context = match CorrelatorContext::new(&m, &gpubox_files) {
        Ok(c) => c,
        Err(e) => {
            set_error_message(
                &format!("{}", e),
                error_message as *mut u8,
                error_message_length,
            );
            // Return failure
            return 1;
        }
    };
    *out_correlator_context_ptr = Box::into_raw(Box::new(context));
    // Return success
    0
}

/// Display an `CorrelatorContext` struct.
///
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must contain an `CorrelatorContext` object already populated via `mwalib_correlator_context_new`
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_context_display(
    correlator_context_ptr: *const CorrelatorContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_context() ERROR: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }

    let context = &*correlator_context_ptr;

    println!("{}", context);

    // Return success
    0
}

/// Read a single timestep / coarse channel of MWA data.
///
/// This method takes as input a timestep_index and a coarse_chan_index to return one
/// HDU of data in [baseline][freq][pol][r][i] format
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
///
/// * `timestep_index` - index within the timestep array for the desired timestep. This corresponds
///                      to TimeStep.get(context, N) where N is timestep_index.
///
/// * `coarse_chan_index` - index within the coarse_chan array for the desired coarse channel. This corresponds
///                            to CoarseChannel.get(context, N) where N is coarse_chan_index.
///
/// * `buffer_ptr` - pointer to caller-owned and allocated buffer to write data into.
///
/// * `buffer_len` - length of `buffer_ptr`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated object from the `mwalib_correlator_context_new` function.
/// * Caller *must* call `mwalib_correlator_context_free_read_buffer` function to release the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_context_read_by_baseline(
    correlator_context_ptr: *mut CorrelatorContext,
    timestep_index: size_t,
    coarse_chan_index: size_t,
    buffer_ptr: *mut c_float,
    buffer_len: size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Load the previously-initialised context and buffer structs. Exit if
    // either of these are null.
    let corr_context = if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_context_read_by_baseline() ERROR: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    } else {
        &mut *correlator_context_ptr
    };

    // Don't do anything if the buffer pointer is null.
    if buffer_ptr.is_null() {
        return 1;
    }

    let output_slice = slice::from_raw_parts_mut(buffer_ptr, buffer_len);

    // Read data in.
    let data = match corr_context.read_by_baseline(timestep_index, coarse_chan_index) {
        Ok(data) => data,
        Err(e) => {
            set_error_message(
                &format!("{}", e),
                error_message as *mut u8,
                error_message_length,
            );
            return 1;
        }
    };

    // If the data buffer is empty, then just return a null pointer.
    if data.is_empty() {
        set_error_message(
            "mwalib_correlator_context_read_by_baseline() ERROR: no data was returned.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }

    // Populate the buffer which was provided to us by caller
    output_slice[..data.len()].copy_from_slice(data.as_slice());
    // Return Success
    0
}

/// Read a single timestep / coarse channel of MWA data.
///
/// This method takes as input a timestep_index and a coarse_chan_index to return one
/// HDU of data in [freq][baseline][pol][r][i] format
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
///
/// * `timestep_index` - index within the timestep array for the desired timestep. This corresponds
///                      to TimeStep.get(context, N) where N is timestep_index.
///
/// * `coarse_chan_index` - index within the coarse_chan array for the desired coarse channel. This corresponds
///                            to CoarseChannel.get(context, N) where N is coarse_chan_index.
///
/// * `buffer_ptr` - pointer to caller-owned and allocated buffer to write data into.
///
/// * `buffer_len` - length of `buffer_ptr`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated object from the `mwalib_correlator_context_new` function.
/// * Caller *must* call `mwalib_correlator_context_free_read_buffer` function to release the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_context_read_by_frequency(
    correlator_context_ptr: *mut CorrelatorContext,
    timestep_index: size_t,
    coarse_chan_index: size_t,
    buffer_ptr: *mut c_float,
    buffer_len: size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Load the previously-initialised context and buffer structs. Exit if
    // either of these are null.
    let corr_context = if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_context_read_by_frequency() ERROR: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    } else {
        &mut *correlator_context_ptr
    };
    // Don't do anything if the buffer pointer is null.
    if buffer_ptr.is_null() {
        return 1;
    }

    let output_slice = slice::from_raw_parts_mut(buffer_ptr, buffer_len);

    // Read data in.
    let data = match corr_context.read_by_frequency(timestep_index, coarse_chan_index) {
        Ok(data) => data,
        Err(e) => {
            set_error_message(
                &format!("{}", e),
                error_message as *mut u8,
                error_message_length,
            );
            return 1;
        }
    };

    // If the data buffer is empty, then just return a null pointer.
    if data.is_empty() {
        set_error_message(
            "mwalib_correlator_context_read_by_frequency() ERROR: no data was returned.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }

    // Populate the buffer which was provided to us by caller
    output_slice[..data.len()].copy_from_slice(data.as_slice());
    // Return Success
    0
}

/// Free a previously-allocated `CorrelatorContext` struct (and it's members).
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `CorrelatorContext` object
/// * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * `correlator_context_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_context_free(
    correlator_context_ptr: *mut CorrelatorContext,
) -> i32 {
    if correlator_context_ptr.is_null() {
        return 0;
    }
    // Release correlator context if applicable
    Box::from_raw(correlator_context_ptr);

    // Return success
    0
}

/// Create and return a pointer to an `VoltageContext` struct based on metafits and voltage files
///
/// # Arguments
///
/// * `metafits_filename` - pointer to char* buffer containing the full path and filename of a metafits file.
///
/// * `voltage_filenames` - pointer to array of char* buffers containing the full path and filename of the voltage files.
///
/// * `voltage_file_count` - length of the voltage char* array.
///
/// * `out_voltage_context_ptr` - A Rust-owned populated `VoltageContext` pointer. Free with `mwalib_voltage_context_free`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated `char*` buffer for any error messages.
/// * Caller *must* call function `mwalib_voltage_context_free` to release the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_context_new(
    metafits_filename: *const c_char,
    voltage_filenames: *mut *const c_char,
    voltage_file_count: size_t,
    out_voltage_context_ptr: &mut *mut VoltageContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    let m = CStr::from_ptr(metafits_filename)
        .to_str()
        .unwrap()
        .to_string();
    let voltage_slice = slice::from_raw_parts(voltage_filenames, voltage_file_count);
    let mut voltage_files = Vec::with_capacity(voltage_file_count);
    for v in voltage_slice {
        let s = CStr::from_ptr(*v).to_str().unwrap();
        voltage_files.push(s.to_string())
    }
    let context = match VoltageContext::new(&m, &voltage_files) {
        Ok(c) => c,
        Err(e) => {
            set_error_message(
                &format!("{}", e),
                error_message as *mut u8,
                error_message_length,
            );
            // Return failure
            return 1;
        }
    };
    *out_voltage_context_ptr = Box::into_raw(Box::new(context));
    // Return success
    0
}

/// Display a `VoltageContext` struct.
///
///
/// # Arguments
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `voltage_context_ptr` must contain an `VoltageContext` object already populated via `mwalib_voltage_context_new`
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_context_display(
    voltage_context_ptr: *const VoltageContext,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if voltage_context_ptr.is_null() {
        set_error_message(
            "mwalib_voltage_context() ERROR: null pointer for voltage_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }

    let context = &*voltage_context_ptr;

    println!("{}", context);

    // Return success
    0
}

/// Free a previously-allocated `VoltageContext` struct (and it's members).
///
/// # Arguments
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `VoltageContext` object
/// * `voltage_context_ptr` must point to a populated `VoltageContext` object from the `mwalib_voltage_context_new` function.
/// * `voltage_context_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_context_free(
    voltage_context_ptr: *mut VoltageContext,
) -> i32 {
    if voltage_context_ptr.is_null() {
        return 0;
    }
    // Release voltage context if applicable
    Box::from_raw(voltage_context_ptr);

    // Return success
    0
}

///
/// This a C struct to allow the caller to consume the metafits metadata
///
#[repr(C)]
pub struct MetafitsMetadata {
    /// Observation id
    pub obs_id: u32,
    /// ATTEN_DB  // global analogue attenuation, in dB
    pub global_analogue_attenuation_db: f64,
    /// RA tile pointing
    pub ra_tile_pointing_deg: f64,
    /// DEC tile pointing
    pub dec_tile_pointing_deg: f64,
    /// RA phase centre
    pub ra_phase_center_deg: f64,
    /// DEC phase centre
    pub dec_phase_center_deg: f64,
    /// AZIMUTH
    pub az_deg: f64,
    /// ALTITUDE
    pub alt_deg: f64,
    /// Zenith angle of the pointing centre in degrees
    pub za_deg: f64,
    /// AZIMUTH of the pointing centre in radians
    pub az_rad: f64,
    /// ALTITUDE (a.k.a. elevation) of the pointing centre in radians
    pub alt_rad: f64,
    /// Zenith angle of the pointing centre in radians
    pub za_rad: f64,
    /// Altitude of Sun
    pub sun_alt_deg: f64,
    /// Distance from pointing center to Sun
    pub sun_distance_deg: f64,
    /// Distance from pointing center to the Moon
    pub moon_distance_deg: f64,
    /// Distance from pointing center to Jupiter
    pub jupiter_distance_deg: f64,
    /// Local Sidereal Time
    pub lst_deg: f64,
    /// Local Sidereal Time in radians
    pub lst_rad: f64,
    /// Hour Angle of pointing center (as a string)
    pub hour_angle_string: *mut c_char,
    /// GRIDNAME
    pub grid_name: *mut c_char,
    /// GRIDNUM
    pub grid_number: i32,
    /// CREATOR
    pub creator: *mut c_char,
    /// PROJECT
    pub project_id: *mut c_char,
    /// Observation name
    pub obs_name: *mut c_char,
    /// MWA observation mode
    pub mode: *mut c_char,
    /// Correlator fine_chan_resolution
    pub corr_fine_chan_width_hz: u32,
    /// Correlator mode dump time
    pub corr_int_time_ms: u64,
    /// Number of fine channels in each coarse channel for a correlator observation
    pub num_corr_fine_chans_per_coarse: usize,
    /// Scheduled start (gps time) of observation
    pub sched_start_utc: i64,
    /// Scheduled end (gps time) of observation
    pub sched_end_utc: i64,
    /// Scheduled start (MJD) of observation
    pub sched_start_mjd: f64,
    /// Scheduled end (MJD) of observation
    pub sched_end_mjd: f64,
    /// Scheduled start (UNIX time) of observation
    pub sched_start_unix_time_ms: u64,
    /// Scheduled end (UNIX time) of observation
    pub sched_end_unix_time_ms: u64,
    /// Scheduled start (GPS) of observation
    pub sched_start_gps_time_ms: u64,
    /// Scheduled end (GPS) of observation
    pub sched_end_gps_time_ms: u64,
    /// Scheduled duration of observation
    pub sched_duration_ms: u64,
    /// Seconds of bad data after observation starts
    pub quack_time_duration_ms: u64,
    /// OBSID+QUACKTIM as Unix timestamp (first good timestep)
    pub good_time_unix_ms: u64,
    /// Good time expressed as GPS seconds
    pub good_time_gps_ms: u64,
    /// Total number of antennas (tiles) in the array
    pub num_ants: usize,
    /// The Metafits defines an rf chain for antennas(tiles) * pol(X,Y)
    pub num_rf_inputs: usize,
    /// Number of antenna pols. e.g. X and Y
    pub num_ant_pols: usize,
    /// Number of baselines
    pub num_baselines: usize,
    /// Number of visibility_pols
    pub num_visibility_pols: usize,
    /// Number of coarse channels we should have
    pub num_coarse_chans: usize,
    /// Total bandwidth of observation assuming we have all coarse channels
    pub obs_bandwidth_hz: u32,
    /// Bandwidth of each coarse channel
    pub coarse_chan_width_hz: u32,
    /// Centre frequency of observation
    pub centre_freq_hz: u32,
    /// filename of metafits file used
    pub metafits_filename: *mut c_char,
}

/// This passed back a struct containing the `MetafitsContext` metadata, given a MetafitsContext, CorrelatorContext or VoltageContext
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object. (Exclusive with correlator_context_ptr and voltage_context_ptr)
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object. (Exclusive with metafits_context_ptr and voltage_context_ptr)
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object. (Exclusive with metafits_context_ptr and correlator_context_ptr)
///
/// * `out_metafits_metadata_ptr` - pointer to a Rust-owned `mwalibMetafitsMetadata` struct. Free with `mwalib_metafits_metadata_free`
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `metafits_context_ptr` must point to a populated MetafitsContext object from the `mwalib_metafits_context_new` function OR
/// * `correlator_context_ptr` must point to a populated CorrelatorContext object from the 'mwalib_correlator_context_new' function OR
/// * `voltage_context_ptr` must point to a populated VoltageContext object from the `mwalib_voltage_context_new` function. (Set the unused contexts to NULL).
/// * Caller must call `mwalib_metafits_metadata_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_metafits_metadata_get(
    metafits_context_ptr: *mut MetafitsContext,
    correlator_context_ptr: *mut CorrelatorContext,
    voltage_context_ptr: *mut VoltageContext,
    out_metafits_metadata_ptr: &mut *mut MetafitsMetadata,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Ensure only either metafits XOR correlator XOR voltage context is passed in
    if !(!metafits_context_ptr.is_null()
        ^ !correlator_context_ptr.is_null()
        ^ !voltage_context_ptr.is_null())
    {
        set_error_message(
            "mwalib_metafits_metadata_get() ERROR: pointers for metafits_context_ptr, correlator_context_ptr and/or voltage_context_ptr were passed in. Only one should be provided.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Create our metafits context pointer depending on what was passed in
    let metafits_context = {
        if !metafits_context_ptr.is_null() {
            // Caller passed in a metafits context, so use that
            &*metafits_context_ptr
        } else if !correlator_context_ptr.is_null() {
            // Caller passed in a correlator context, so use that
            &(*correlator_context_ptr).metafits_context
        } else {
            // Caller passed in a voltage context, so use that
            &(*voltage_context_ptr).metafits_context
        }
    };

    // Populate the outgoing structure with data from the metafits context
    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    let out_context = {
        let MetafitsContext {
            obs_id,
            sched_start_gps_time_ms,
            sched_end_gps_time_ms,
            sched_start_unix_time_ms,
            sched_end_unix_time_ms,
            sched_start_utc,
            sched_end_utc,
            sched_start_mjd,
            sched_end_mjd,
            sched_duration_ms,
            ra_tile_pointing_degrees,
            dec_tile_pointing_degrees,
            ra_phase_center_degrees,
            dec_phase_center_degrees,
            az_deg,
            alt_deg,
            za_deg,
            az_rad,
            alt_rad,
            za_rad,
            sun_alt_deg,
            sun_distance_deg,
            moon_distance_deg,
            jupiter_distance_deg,
            lst_deg: lst_degrees,
            lst_rad: lst_radians,
            hour_angle_string,
            grid_name,
            grid_number,
            creator,
            project_id,
            obs_name,
            mode,
            corr_fine_chan_width_hz,
            corr_int_time_ms,
            num_corr_fine_chans_per_coarse,
            receivers: _, // Not currently supported via FFI
            delays: _,    // Not currently supported via FFI
            global_analogue_attenuation_db,
            quack_time_duration_ms,
            good_time_unix_ms,
            good_time_gps_ms,
            num_ants,
            antennas: _, // This is provided by the seperate antenna struct in FFI
            num_rf_inputs,
            rf_inputs: _, // This is provided by the seperate rfinput struct in FFI
            num_ant_pols,
            num_baselines,
            baselines: _, // This is provided by the seperate baseline struct in FFI
            num_visibility_pols,
            visibility_pols: _, // This is provided by the seperate visibility_pol struct in FFI
            num_coarse_chans,
            obs_bandwidth_hz,
            coarse_chan_width_hz,
            centre_freq_hz,
            metafits_filename,
        } = metafits_context;
        MetafitsMetadata {
            obs_id: *obs_id,
            global_analogue_attenuation_db: *global_analogue_attenuation_db,
            ra_tile_pointing_deg: *ra_tile_pointing_degrees,
            dec_tile_pointing_deg: *dec_tile_pointing_degrees,
            ra_phase_center_deg: (*ra_phase_center_degrees).unwrap_or(0.),
            dec_phase_center_deg: (*dec_phase_center_degrees).unwrap_or(0.),
            az_deg: *az_deg,
            alt_deg: *alt_deg,
            za_deg: *za_deg,
            az_rad: *az_rad,
            alt_rad: *alt_rad,
            za_rad: *za_rad,
            sun_alt_deg: *sun_alt_deg,
            sun_distance_deg: *sun_distance_deg,
            moon_distance_deg: *moon_distance_deg,
            jupiter_distance_deg: *jupiter_distance_deg,
            lst_deg: *lst_degrees,
            lst_rad: *lst_radians,
            hour_angle_string: CString::new(String::from(&*hour_angle_string))
                .unwrap()
                .into_raw(),
            grid_name: CString::new(String::from(&*grid_name)).unwrap().into_raw(),
            grid_number: *grid_number,
            creator: CString::new(String::from(&*creator)).unwrap().into_raw(),
            project_id: CString::new(String::from(&*project_id)).unwrap().into_raw(),
            obs_name: CString::new(String::from(&*obs_name)).unwrap().into_raw(),
            mode: CString::new(String::from(&*mode)).unwrap().into_raw(),
            corr_fine_chan_width_hz: *corr_fine_chan_width_hz,
            corr_int_time_ms: *corr_int_time_ms,
            num_corr_fine_chans_per_coarse: *num_corr_fine_chans_per_coarse,
            sched_start_utc: sched_start_utc.timestamp(),
            sched_end_utc: sched_end_utc.timestamp(),
            sched_start_mjd: *sched_start_mjd,
            sched_end_mjd: *sched_end_mjd,
            sched_start_unix_time_ms: *sched_start_unix_time_ms,
            sched_end_unix_time_ms: *sched_end_unix_time_ms,
            sched_start_gps_time_ms: *sched_start_gps_time_ms,
            sched_end_gps_time_ms: *sched_end_gps_time_ms,
            sched_duration_ms: *sched_duration_ms,
            quack_time_duration_ms: *quack_time_duration_ms,
            good_time_unix_ms: *good_time_unix_ms,
            good_time_gps_ms: *good_time_gps_ms,
            num_ants: *num_ants,
            num_rf_inputs: *num_rf_inputs,
            num_ant_pols: *num_ant_pols,
            num_baselines: *num_baselines,
            num_visibility_pols: *num_visibility_pols,
            num_coarse_chans: *num_coarse_chans,
            obs_bandwidth_hz: *obs_bandwidth_hz,
            coarse_chan_width_hz: *coarse_chan_width_hz,
            centre_freq_hz: *centre_freq_hz,
            metafits_filename: CString::new(String::from(&*metafits_filename))
                .unwrap()
                .into_raw(),
        }
    };

    // Pass back a pointer to the rust owned struct
    *out_metafits_metadata_ptr = Box::into_raw(Box::new(out_context));

    // Return Success
    0
}

/// Free a previously-allocated `mwalibMetafitsMetadata` struct.
///
/// # Arguments
///
/// * `metafits_metadata_ptr` - pointer to an already populated `mwalibMetafitsMetadata` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `mwalibMetafitsMetadata` object
/// * `metafits_metadata_ptr` must point to a populated `mwalibMetafitsMetadata` object from the `mwalib_metafits_metadata_get` function.
/// * `metafits_metadata_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_metafits_metadata_free(
    metafits_metadata_ptr: *mut MetafitsMetadata,
) -> i32 {
    // If the pointer is null, just return
    if metafits_metadata_ptr.is_null() {
        return 0;
    }
    drop(Box::from_raw(metafits_metadata_ptr));

    // Return success
    0
}

///
/// C Representation of the `CorrelatorContext` metadata
///
#[repr(C)]
pub struct CorrelatorMetadata {
    /// Version of the correlator format
    pub corr_version: CorrelatorVersion,
    /// The proper start of the observation (the time that is common to all
    /// provided gpubox files).
    pub start_unix_time_ms: u64,
    /// `end_time_ms` will is the actual end time of the observation
    /// i.e. start time of last common timestep plus integration time.
    pub end_unix_time_ms: u64,
    /// `start_unix_time_ms` but in GPS milliseconds
    pub start_gps_time_ms: u64,
    /// `end_unix_time_ms` but in GPS milliseconds
    pub end_gps_time_ms: u64,
    /// Total duration of observation (based on gpubox files)
    pub duration_ms: u64,
    /// Number of timesteps in the observation
    pub num_timesteps: usize,
    /// Number of coarse channels
    pub num_coarse_chans: usize,
    /// Total bandwidth of observation (of the coarse channels we have)
    pub bandwidth_hz: u32,
    /// The number of bytes taken up by a scan/timestep in each gpubox file.
    pub num_timestep_coarse_chan_bytes: usize,
    /// The number of floats in each gpubox HDU.
    pub num_timestep_coarse_chan_floats: usize,
    /// This is the number of gpubox files *per batch*.
    pub num_gpubox_files: usize,
}

/// This returns a struct containing the `CorrelatorContext` metadata
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
///
/// * `out_correaltor_metadata_ptr` - A Rust-owned populated `CorrelatorMetadata` struct. Free with `mwalib_correlator_metadata_free`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * Caller must call `mwalib_correlator_metadata_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_metadata_get(
    correlator_context_ptr: *mut CorrelatorContext,
    out_correlator_metadata_ptr: &mut *mut CorrelatorMetadata,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_metadata_get() ERROR: Warning: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Get the correlator context object from the raw pointer passed in
    let context = &*correlator_context_ptr;

    // Populate the rust owned data structure with data from the correlator context
    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    let out_context = {
        let CorrelatorContext {
            metafits_context: _, // This is provided by the seperate metafits_metadata struct in FFI
            corr_version,
            start_unix_time_ms,
            end_unix_time_ms,
            start_gps_time_ms,
            end_gps_time_ms,
            duration_ms,
            num_timesteps,
            timesteps: _, // This is provided by the seperate timestep struct in FFI
            num_coarse_chans,
            coarse_chans: _, // This is provided by the seperate coarse_chan struct in FFI
            bandwidth_hz,
            num_timestep_coarse_chan_bytes,
            num_timestep_coarse_chan_floats,
            num_gpubox_files,
            gpubox_batches: _, // This is currently not provided to FFI as it is private
            gpubox_time_map: _, // This is currently not provided to FFI as it is private
            legacy_conversion_table: _, // This is currently not provided to FFI as it is private
        } = context;
        CorrelatorMetadata {
            corr_version: *corr_version,
            start_unix_time_ms: *start_unix_time_ms,
            end_unix_time_ms: *end_unix_time_ms,
            start_gps_time_ms: *start_gps_time_ms,
            end_gps_time_ms: *end_gps_time_ms,
            duration_ms: *duration_ms,
            num_timesteps: *num_timesteps,
            num_coarse_chans: *num_coarse_chans,
            bandwidth_hz: *bandwidth_hz,
            num_timestep_coarse_chan_bytes: *num_timestep_coarse_chan_bytes,
            num_timestep_coarse_chan_floats: *num_timestep_coarse_chan_floats,
            num_gpubox_files: *num_gpubox_files,
        }
    };

    // Pass out the pointer to the rust owned data structure
    *out_correlator_metadata_ptr = Box::into_raw(Box::new(out_context));

    // Return success
    0
}

/// Free a previously-allocated `CorrelatorMetadata` struct.
///
/// # Arguments
///
/// * `correlator_metadata_ptr` - pointer to an already populated `CorrelatorMetadata` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `CorrelatorMetadata` object
/// * `correlator_metadata_ptr` must point to a populated `CorrelatorMetadata` object from the `mwalib_correlator_metadata_get` function.
/// * `correlator_metadata_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_metadata_free(
    correlator_metadata_ptr: *mut CorrelatorMetadata,
) -> i32 {
    if correlator_metadata_ptr.is_null() {
        return 0;
    }
    drop(Box::from_raw(correlator_metadata_ptr));

    // Return success
    0
}

///
/// C Representation of the `VoltageContext` metadata
///
#[repr(C)]
pub struct VoltageMetadata {
    /// Version of the correlator format
    pub corr_version: CorrelatorVersion,
    /// The proper start of the observation (the time that is common to all
    /// provided voltage files).
    pub start_gps_time_ms: u64,
    /// `end_gps_time_ms` is the actual end time of the observation    
    /// i.e. start time of last common timestep plus length of a voltage file (1 sec for MWA Legacy, 8 secs for MWAX).
    pub end_gps_time_ms: u64,
    /// `start_gps_time_ms` but in UNIX time (milliseconds)
    pub start_unix_time_ms: u64,
    /// `end_gps_time_ms` but in UNIX time (milliseconds)
    pub end_unix_time_ms: u64,
    /// Total duration of observation (based on voltage files)
    pub duration_ms: u64,
    /// Number of timesteps in the observation
    pub num_timesteps: usize,
    /// The number of millseconds interval between timestep indices
    pub timestep_duration_ms: u64,
    /// The number of samples in each timestep
    pub num_samples_per_timestep: usize,
    /// Number of coarse channels after we've validated the input voltage files
    pub num_coarse_chans: usize,
    /// Total bandwidth of observation (of the coarse channels we have)
    pub bandwidth_hz: u32,
    /// Bandwidth of each coarse channel
    pub coarse_chan_width_hz: u32,
    /// Volatge fine_chan_resolution (if applicable- MWA legacy is 10 kHz, MWAX is unchannelised i.e. the full coarse channel width)
    pub fine_chan_width_hz: u32,
    /// Number of fine channels in each coarse channel
    pub num_fine_chans_per_coarse: usize,
}

/// This returns a struct containing the `VoltageContext` metadata
///
/// # Arguments
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object.
///
/// * `out_voltage_metadata_ptr` - A Rust-owned populated `VoltageMetadata` struct. Free with `mwalib_voltage_metadata_free`.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `voltage_context_ptr` must point to a populated `VoltageContext` object from the `mwalib_voltage_context_new` function.
/// * Caller must call `mwalib_voltage_metadata_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_metadata_get(
    voltage_context_ptr: *mut VoltageContext,
    out_voltage_metadata_ptr: &mut *mut VoltageMetadata,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if voltage_context_ptr.is_null() {
        set_error_message(
            "mwalib_voltage_metadata_get() ERROR: Warning: null pointer for voltage_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Get the voltage context object from the raw pointer passed in
    let context = &*voltage_context_ptr;

    // Populate the rust owned data structure with data from the voltage context
    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    let out_context = {
        let VoltageContext {
            metafits_context: _, // This is provided by the seperate metafits_metadata struct in FFI
            corr_version,
            start_gps_time_ms,
            end_gps_time_ms,
            start_unix_time_ms,
            end_unix_time_ms,
            duration_ms,
            num_timesteps,
            timesteps: _, // This is provided by the seperate timestep struct in FFI
            timestep_duration_ms,
            num_samples_per_timestep,
            num_coarse_chans,
            coarse_chans: _, // This is provided by the seperate coarse_chan struct in FFI
            bandwidth_hz,
            coarse_chan_width_hz,
            fine_chan_width_hz,
            num_fine_chans_per_coarse,
            voltage_batches: _, // This is currently not provided to FFI as it is private
            voltage_time_map: _, // This is currently not provided to FFI as it is private
        } = context;
        VoltageMetadata {
            corr_version: *corr_version,
            start_gps_time_ms: *start_gps_time_ms,
            end_gps_time_ms: *end_gps_time_ms,
            start_unix_time_ms: *start_unix_time_ms,
            end_unix_time_ms: *end_unix_time_ms,
            duration_ms: *duration_ms,
            num_timesteps: *num_timesteps,
            timestep_duration_ms: *timestep_duration_ms,
            num_samples_per_timestep: *num_samples_per_timestep,
            num_coarse_chans: *num_coarse_chans,
            bandwidth_hz: *bandwidth_hz,
            coarse_chan_width_hz: *coarse_chan_width_hz,
            fine_chan_width_hz: *fine_chan_width_hz,
            num_fine_chans_per_coarse: *num_fine_chans_per_coarse,
        }
    };

    // Pass out the pointer to the rust owned data structure
    *out_voltage_metadata_ptr = Box::into_raw(Box::new(out_context));

    // Return success
    0
}

/// Free a previously-allocated `VoltageMetadata` struct.
///
/// # Arguments
///
/// * `voltage_metadata_ptr` - pointer to an already populated `VoltageMetadata` object
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `VoltageMetadata` object
/// * `voltage_metadata_ptr` must point to a populated `VoltageMetadata` object from the `mwalib_voltage_metadata_get` function.
/// * `voltage_metadata_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_metadata_free(
    voltage_metadata_ptr: *mut VoltageMetadata,
) -> i32 {
    if voltage_metadata_ptr.is_null() {
        return 0;
    }
    drop(Box::from_raw(voltage_metadata_ptr));

    // Return success
    0
}

/// Representation in C of an `Antenna` struct
#[repr(C)]
pub struct Antenna {
    /// This is the antenna number.
    /// Nominally this is the field we sort by to get the desired output order of antenna.
    /// X and Y have the same antenna number. This is the sorted ordinal order of the antenna.None
    /// e.g. 0...N-1
    pub ant: u32,
    /// Numeric part of tile_name for the antenna. Each pol has the same value
    /// e.g. tile_name "tile011" hsa tile_id of 11
    pub tile_id: u32,
    /// Human readable name of the antenna
    /// X and Y have the same name
    pub tile_name: *mut c_char,
    /// Index within the array of rfinput structs of the x pol
    pub rfinput_x: usize,
    /// Index within the array of rfinput structs of the y pol
    pub rfinput_y: usize,
}

/// This passes back an array of structs containing all antennas given a metafits OR correlator context.
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object. (Exclusive with `correlator_context_ptr` and `voltage_context_ptr`)
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object. (Exclusive with `metafits_context_ptr` and `voltage_context_ptr`)
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object. (Exclusive with `metafits_context_ptr` and `correlator_context_ptr`)
///
/// * `out_ants_ptr` - A Rust-owned populated array of `Antenna` struct. Free with `mwalib_antennas_free`.
///
/// * `out_ants_len` - Antennas array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `metafits_context_ptr` must point to a populated MetafitsContext object from the `mwalib_metafits_context_new` function.
/// * Caller must call `mwalib_antenna_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_antennas_get(
    metafits_context_ptr: *mut MetafitsContext,
    correlator_context_ptr: *mut CorrelatorContext,
    voltage_context_ptr: *mut VoltageContext,
    out_ants_ptr: &mut *mut Antenna,
    out_ants_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Ensure only either metafits XOR correlator XOR voltage context is passed in
    if !(!metafits_context_ptr.is_null()
        ^ !correlator_context_ptr.is_null()
        ^ !voltage_context_ptr.is_null())
    {
        set_error_message(
            "mwalib_antennas_get() ERROR: pointers for metafits_context_ptr, correlator_context_ptr and/or voltage_context_ptr were passed in. Only one should be provided.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Create our metafits context pointer depending on what was passed in
    let metafits_context = {
        if !metafits_context_ptr.is_null() {
            // Caller passed in a metafits context, so use that
            &*metafits_context_ptr
        } else if !correlator_context_ptr.is_null() {
            // Caller passed in a correlator context, so use that
            &(*correlator_context_ptr).metafits_context
        } else {
            // Caller passed in a voltage context, so use that
            &(*voltage_context_ptr).metafits_context
        }
    };

    let mut item_vec: Vec<Antenna> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in metafits_context.antennas.iter() {
        let out_item = {
            let antenna::Antenna {
                ant,
                tile_id,
                tile_name,
                rfinput_x,
                rfinput_y,
            } = item;
            Antenna {
                ant: *ant,
                tile_id: *tile_id,
                tile_name: CString::new(tile_name.as_str()).unwrap().into_raw(),
                rfinput_x: rfinput_x.subfile_order as usize,
                rfinput_y: rfinput_y.subfile_order as usize,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_ants_len = item_vec.len();
    *out_ants_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// Free a previously-allocated `Antenna` array of structs.
///
/// # Arguments
///
/// * `ants_ptr` - pointer to an already populated `Antenna` array
///
/// * `ants_len` - number of elements in the pointed to array
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `Antenna` array
/// * `ants_ptr` must point to a populated `Antenna` array from the `mwalib_antennas_get` function.
/// * `ants_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_antennas_free(ants_ptr: *mut Antenna, ants_len: size_t) -> i32 {
    if ants_ptr.is_null() {
        return 0;
    }

    // Extract a slice from the pointer
    let slice: &mut [Antenna] = slice::from_raw_parts_mut(ants_ptr, ants_len);
    // Now for each item we need to free anything on the heap
    for i in slice.iter_mut() {
        drop(Box::from_raw(i.tile_name));
    }

    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}

///
/// C Representation of a `Baseline` struct
///
#[repr(C)]
pub struct Baseline {
    /// Index in the `MetafitsContext` antenna array for antenna1 for this baseline
    pub ant1_index: usize,
    /// Index in the `MetafitsContext` antenna array for antenna2 for this baseline
    pub ant2_index: usize,
}

/// This passes a pointer to an array of baselines
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object. (Exclusive with `correlator_context_ptr` and `voltage_context_ptr`)
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object. (Exclusive with `metafits_context_ptr` and `voltage_context_ptr`)
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object. (Exclusive with `metafits_context_ptr` and `correlator_context_ptr`)
///
/// * `out_baselines_ptr` - populated, array of rust-owned baseline structs. Free with `mwalib_baselines_free`.
///
/// * `out_baselines_len` - baseline array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * Caller must call `mwalib_baselines_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_baselines_get(
    metafits_context_ptr: *mut MetafitsContext,
    correlator_context_ptr: *mut CorrelatorContext,
    voltage_context_ptr: *mut VoltageContext,
    out_baselines_ptr: &mut *mut Baseline,
    out_baselines_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Ensure only either metafits XOR correlator XOR voltage context is passed in
    if !(!metafits_context_ptr.is_null()
        ^ !correlator_context_ptr.is_null()
        ^ !voltage_context_ptr.is_null())
    {
        set_error_message(
            "mwalib_baselines_get() ERROR: pointers for metafits_context_ptr, correlator_context_ptr and/or voltage_context_ptr were passed in. Only one should be provided.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Create our metafits context pointer depending on what was passed in
    let metafits_context = {
        if !metafits_context_ptr.is_null() {
            // Caller passed in a metafits context, so use that
            &*metafits_context_ptr
        } else if !correlator_context_ptr.is_null() {
            // Caller passed in a correlator context, so use that
            &(*correlator_context_ptr).metafits_context
        } else {
            // Caller passed in a voltage context, so use that
            &(*voltage_context_ptr).metafits_context
        }
    };

    let mut item_vec: Vec<Baseline> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in metafits_context.baselines.iter() {
        let out_item = {
            let baseline::Baseline {
                ant1_index,
                ant2_index,
            } = item;
            Baseline {
                ant1_index: *ant1_index,
                ant2_index: *ant2_index,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_baselines_len = item_vec.len();
    *out_baselines_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// Free a previously-allocated `Baseline` struct.
///
/// # Arguments
///
/// * `baselines_ptr` - pointer to an already populated `Baseline` array
///
/// * `baselines_len` - number of elements in the pointed to array
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `Baseline` array
/// * `baseline_ptr` must point to a populated `Baseline` array from the `mwalib_baselines_get` function.
/// * `baseline_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_baselines_free(
    baselines_ptr: *mut Baseline,
    baselines_len: size_t,
) -> i32 {
    if baselines_ptr.is_null() {
        return 0;
    }
    // Extract a slice from the pointer
    let slice: &mut [Baseline] = slice::from_raw_parts_mut(baselines_ptr, baselines_len);

    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}

/// Representation in C of an `CoarseChannel` struct
#[repr(C)]
pub struct CoarseChannel {
    /// Correlator channel is 0 indexed (0..N-1)
    pub corr_chan_number: usize,
    /// Receiver channel is 0-255 in the RRI recivers
    pub rec_chan_number: usize,
    /// gpubox channel number
    /// Legacy e.g. obsid_datetime_gpuboxXX_00
    /// v2     e.g. obsid_datetime_gpuboxXXX_00
    pub gpubox_number: usize,
    /// Width of a coarse channel in Hz
    pub chan_width_hz: u32,
    /// Starting frequency of coarse channel in Hz
    pub chan_start_hz: u32,
    /// Centre frequency of coarse channel in Hz
    pub chan_centre_hz: u32,
    /// Ending frequency of coarse channel in Hz
    pub chan_end_hz: u32,
}

/// This passes a pointer to an array of correlator coarse channel
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
///
/// * `out_coarse_chans_ptr` - A Rust-owned populated `CoarseChannel` array of structs. Free with `mwalib_coarse_channels_free`.
///
/// * `out_coarse_chans_len` - Coarse channel array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated `mwalibCorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * Caller must call `mwalib_coarse_channels_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_coarse_channels_get(
    correlator_context_ptr: *mut CorrelatorContext,
    out_coarse_chans_ptr: &mut *mut CoarseChannel,
    out_coarse_chans_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_coarse_channels_get() ERROR: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    let context = &*correlator_context_ptr;

    let mut item_vec: Vec<CoarseChannel> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in context.coarse_chans.iter() {
        let out_item = {
            let coarse_channel::CoarseChannel {
                corr_chan_number,
                rec_chan_number,
                gpubox_number,
                chan_width_hz,
                chan_start_hz,
                chan_centre_hz,
                chan_end_hz,
            } = item;
            CoarseChannel {
                corr_chan_number: *corr_chan_number,
                rec_chan_number: *rec_chan_number,
                gpubox_number: *gpubox_number,
                chan_width_hz: *chan_width_hz,
                chan_start_hz: *chan_start_hz,
                chan_centre_hz: *chan_centre_hz,
                chan_end_hz: *chan_end_hz,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_coarse_chans_len = item_vec.len();
    *out_coarse_chans_ptr = ffi_array_to_boxed_slice(item_vec);

    // return success
    0
}

/// This passes a pointer to an array of voltage coarse channel
///
/// # Arguments
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object.
///
/// * `out_coarse_chans_ptr` - A Rust-owned populated `CoarseChannel` array of structs. Free with `mwalib_coarse_channels_free`.
///
/// * `out_coarse_chans_len` - Coarse channel array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `voltage_context_ptr` must point to a populated `mwalibVoltageContext` object from the `mwalib_voltage_context_new` function.
/// * Caller must call `mwalib_coarse_channels_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_coarse_channels_get(
    voltage_context_ptr: *mut VoltageContext,
    out_coarse_chans_ptr: &mut *mut CoarseChannel,
    out_coarse_chans_len: &mut usize,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if voltage_context_ptr.is_null() {
        set_error_message(
            "mwalib_voltage_coarse_channels_get() ERROR: null pointer for voltage_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    let context = &*voltage_context_ptr;

    let mut item_vec: Vec<CoarseChannel> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in context.coarse_chans.iter() {
        let out_item = {
            let coarse_channel::CoarseChannel {
                corr_chan_number,
                rec_chan_number,
                gpubox_number,
                chan_width_hz,
                chan_start_hz,
                chan_centre_hz,
                chan_end_hz,
            } = item;
            CoarseChannel {
                corr_chan_number: *corr_chan_number,
                rec_chan_number: *rec_chan_number,
                gpubox_number: *gpubox_number,
                chan_width_hz: *chan_width_hz,
                chan_start_hz: *chan_start_hz,
                chan_centre_hz: *chan_centre_hz,
                chan_end_hz: *chan_end_hz,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_coarse_chans_len = item_vec.len();
    *out_coarse_chans_ptr = ffi_array_to_boxed_slice(item_vec);

    // return success
    0
}

/// Free a previously-allocated `CoarseChannel` struct.
///
/// # Arguments
///
/// * `coarse_chans_ptr` - pointer to an already populated `CoarseChannel` array
///
/// * `coarse_chans_len` - number of elements in the pointed to array
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `CoarseChannel` array
/// * `coarse_chan_ptr` must point to a populated `CoarseChannel` array from the `mwalib_correlator_coarse_channels_get` function.
/// * `coarse_chan_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_coarse_channels_free(
    coarse_chans_ptr: *mut CoarseChannel,
    coarse_chans_len: size_t,
) -> i32 {
    if coarse_chans_ptr.is_null() {
        return 0;
    }
    // Extract a slice from the pointer
    let slice: &mut [CoarseChannel] = slice::from_raw_parts_mut(coarse_chans_ptr, coarse_chans_len);
    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}

/// Representation in C of an `RFInput` struct
#[repr(C)]
pub struct Rfinput {
    /// This is the metafits order (0-n inputs)
    pub input: u32,
    /// This is the antenna number.
    /// Nominally this is the field we sort by to get the desired output order of antenna.
    /// X and Y have the same antenna number. This is the sorted ordinal order of the antenna.None
    /// e.g. 0...N-1
    pub ant: u32,
    /// Numeric part of tile_name for the antenna. Each pol has the same value
    /// e.g. tile_name "tile011" hsa tile_id of 11
    pub tile_id: u32,
    /// Human readable name of the antenna
    /// X and Y have the same name
    pub tile_name: *mut c_char,
    /// Polarisation - X or Y
    pub pol: *mut c_char,
    /// Electrical length in metres for this antenna and polarisation to the receiver
    pub electrical_length_m: f64,
    /// Antenna position North from the array centre (metres)
    pub north_m: f64,
    /// Antenna position East from the array centre (metres)
    pub east_m: f64,
    /// Antenna height from the array centre (metres)
    pub height_m: f64,
    /// AKA PFB to correlator input order (only relevant for pre V2 correlator)
    pub vcs_order: u32,
    /// Subfile order is the order in which this rf_input is desired in our final output of data
    pub subfile_order: u32,
    /// Is this rf_input flagged out (due to tile error, etc from metafits)
    pub flagged: bool,
    /// Receiver number
    pub rec_number: u32,
    /// Receiver slot number
    pub rec_slot_number: u32,
}

/// This passes a pointer to an array of antenna given a metafits context OR correlator context
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object. (Exclusive with `correlator_context_ptr` and `voltage_context_ptr`)
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object. (Exclusive with `metafits_context_ptr` and `voltage_context_ptr`)
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object. (Exclusive with `metafits_context_ptr` and `correlator_context_ptr`)
///
/// * `out_rfinputs_ptr` - A Rust-owned populated `RFInput` array of structs. Free with `mwalib_rfinputs_free`.
///
/// * `out_rfinputs_len` - rfinputs array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `metafits_context_ptr` must point to a populated `MetafitsContext` object from the `mwalib_metafits_context_new` function.
/// * Caller must call `mwalib_rfinputs_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_rfinputs_get(
    metafits_context_ptr: *mut MetafitsContext,
    correlator_context_ptr: *mut CorrelatorContext,
    voltage_context_ptr: *mut VoltageContext,
    out_rfinputs_ptr: &mut *mut Rfinput,
    out_rfinputs_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Ensure only either metafits XOR correlator XOR voltage context is passed in
    if !(!metafits_context_ptr.is_null()
        ^ !correlator_context_ptr.is_null()
        ^ !voltage_context_ptr.is_null())
    {
        set_error_message(
            "mwalib_rfinputs_get() ERROR: pointers for metafits_context_ptr, correlator_context_ptr and/or voltage_context_ptr were passed in. Only one should be provided.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Create our metafits context pointer depending on what was passed in
    let metafits_context = {
        if !metafits_context_ptr.is_null() {
            // Caller passed in a metafits context, so use that
            &*metafits_context_ptr
        } else if !correlator_context_ptr.is_null() {
            // Caller passed in a correlator context, so use that
            &(*correlator_context_ptr).metafits_context
        } else {
            // Caller passed in a voltage context, so use that
            &(*voltage_context_ptr).metafits_context
        }
    };

    let mut item_vec: Vec<Rfinput> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in metafits_context.rf_inputs.iter() {
        let out_item = {
            let rfinput::Rfinput {
                input,
                ant,
                tile_id,
                tile_name,
                pol,
                electrical_length_m,
                north_m,
                east_m,
                height_m,
                vcs_order,
                subfile_order,
                flagged,
                rec_number,
                rec_slot_number,
                digital_gains: _, // not currently supported via FFI interface
                dipole_gains: _,  // not currently supported via FFI interface
                dipole_delays: _, // not currently supported via FFI interface
            } = item;
            Rfinput {
                input: *input,
                ant: *ant,
                tile_id: *tile_id,
                tile_name: CString::new(String::from(&*tile_name)).unwrap().into_raw(),
                pol: CString::new(pol.to_string()).unwrap().into_raw(),
                electrical_length_m: *electrical_length_m,
                north_m: *north_m,
                east_m: *east_m,
                height_m: *height_m,
                vcs_order: *vcs_order,
                subfile_order: *subfile_order,
                flagged: *flagged,
                rec_number: *rec_number,
                rec_slot_number: *rec_slot_number,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_rfinputs_len = item_vec.len();
    *out_rfinputs_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// Free a previously-allocated `RFInput` struct.
///
/// # Arguments
///
/// * `rf_inputs_ptr` - pointer to an already populated `RFInput` object
///
/// * `rf_inputs_len` - number of elements in the pointed to array
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `RFInput` array
/// * `rf_input_ptr` must point to a populated `RFInput` array from the `mwalib_rfinputs_get` function.
/// * `rf_input_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_rfinputs_free(
    rf_inputs_ptr: *mut Rfinput,
    rf_inputs_len: size_t,
) -> i32 {
    if rf_inputs_ptr.is_null() {
        return 0;
    }
    // Extract a slice from the pointer
    let slice: &mut [Rfinput] = slice::from_raw_parts_mut(rf_inputs_ptr, rf_inputs_len);
    // Now for each item we need to free anything on the heap
    for i in slice.iter_mut() {
        drop(Box::from_raw(i.tile_name));
        drop(Box::from_raw(i.pol));
    }

    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}

///
/// C Representation of a `TimeStep` struct
///
#[repr(C)]
pub struct TimeStep {
    /// UNIX time (in milliseconds to avoid floating point inaccuracy)
    pub unix_time_ms: u64,
    pub gps_time_ms: u64,
}

/// This passes a pointer to an array of timesteps
///
/// # Arguments
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
///
/// * `out_timesteps_ptr` - A Rust-owned populated `TimeStep` struct. Free with `mwalib_timestep_free`.
///
/// * `out_timesteps_len` - Timesteps array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * Caller must call `mwalib_timestep_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_correlator_timesteps_get(
    correlator_context_ptr: *mut CorrelatorContext,
    out_timesteps_ptr: &mut *mut TimeStep,
    out_timesteps_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if correlator_context_ptr.is_null() {
        set_error_message(
            "mwalib_correlator_timesteps_get() ERROR: null pointer for correlator_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    let context = &*correlator_context_ptr;

    let mut item_vec: Vec<TimeStep> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in context.timesteps.iter() {
        let out_item = {
            let timestep::TimeStep {
                unix_time_ms,
                gps_time_ms,
            } = item;
            TimeStep {
                unix_time_ms: *unix_time_ms,
                gps_time_ms: *gps_time_ms,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_timesteps_len = item_vec.len();
    *out_timesteps_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// This passes a pointer to an array of timesteps
///
/// # Arguments
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object.
///
/// * `out_timesteps_ptr` - A Rust-owned populated `TimeStep` struct. Free with `mwalib_timestep_free`.
///
/// * `out_timesteps_len` - Timesteps array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `voltage_context_ptr` must point to a populated `VoltageContext` object from the `mwalib_voltage_context_new` function.
/// * Caller must call `mwalib_timestep_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_voltage_timesteps_get(
    voltage_context_ptr: *mut VoltageContext,
    out_timesteps_ptr: &mut *mut TimeStep,
    out_timesteps_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    if voltage_context_ptr.is_null() {
        set_error_message(
            "mwalib_voltage_timesteps_get() ERROR: null pointer for voltage_context_ptr passed in",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    let context = &*voltage_context_ptr;

    let mut item_vec: Vec<TimeStep> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in context.timesteps.iter() {
        let out_item = {
            let timestep::TimeStep {
                unix_time_ms,
                gps_time_ms,
            } = item;
            TimeStep {
                unix_time_ms: *unix_time_ms,
                gps_time_ms: *gps_time_ms,
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_timesteps_len = item_vec.len();
    *out_timesteps_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// Free a previously-allocated `TimeStep` struct.
///
/// # Arguments
///
/// * `timesteps_ptr` - pointer to an already populated `TimeStep` array
///
/// * `timesteps_len` - number of elements in the pointed to array
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `TimeStep` array
/// * `timestep_ptr` must point to a populated `TimeStep` array from the `mwalib_correlator_timesteps_get` function.
/// * `timestep_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_timesteps_free(
    timesteps_ptr: *mut TimeStep,
    timesteps_len: size_t,
) -> i32 {
    if timesteps_ptr.is_null() {
        return 0;
    }
    // Extract a slice from the pointer
    let slice: &mut [TimeStep] = slice::from_raw_parts_mut(timesteps_ptr, timesteps_len);
    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}

///
/// C Representation of a `VisibilityPol` struct
///
#[repr(C)]
pub struct VisibilityPol {
    /// Polarisation (e.g. "XX" or "XY" or "YX" or "YY")
    pub polarisation: *mut c_char,
}

/// This passes back a pointer to an array of all visibility polarisations
///
/// # Arguments
///
/// * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object. (Exclusive with `correlator_context_ptr` and `voltage_context_ptr`)
///
/// * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object. (Exclusive with `metafits_context_ptr` and `voltage_context_ptr`)
///
/// * `voltage_context_ptr` - pointer to an already populated `VoltageContext` object. (Exclusive with `metafits_context_ptr` and `correlator_context_ptr`)
///
/// * `out_visibility_pols_ptr` - A Rust-owned populated array of `VisibilityPol` structs. Free with `mwalib_visibility_pols_free`.
///
/// * `out_visibility_pols_len` - Visibility Pols array length.
///
/// * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
///
/// * `error_message_length` - length of error_message char* buffer.
///
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * `error_message` *must* point to an already allocated char* buffer for any error messages.
/// * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
/// * Caller must call `mwalib_visibility_pols_free` once finished, to free the rust memory.
#[no_mangle]
pub unsafe extern "C" fn mwalib_visibility_pols_get(
    metafits_context_ptr: *mut MetafitsContext,
    correlator_context_ptr: *mut CorrelatorContext,
    voltage_context_ptr: *mut VoltageContext,
    out_visibility_pols_ptr: &mut *mut VisibilityPol,
    out_visibility_pols_len: &mut size_t,
    error_message: *const c_char,
    error_message_length: size_t,
) -> i32 {
    // Ensure only either metafits XOR correlator XOR voltage context is passed in
    if !(!metafits_context_ptr.is_null()
        ^ !correlator_context_ptr.is_null()
        ^ !voltage_context_ptr.is_null())
    {
        set_error_message(
            "mwalib_visibility_pols_get() ERROR: pointers for metafits_context_ptr, correlator_context_ptr and/or voltage_context_ptr were passed in. Only one should be provided.",
            error_message as *mut u8,
            error_message_length,
        );
        return 1;
    }
    // Create our metafits context pointer depending on what was passed in
    let metafits_context = {
        if !metafits_context_ptr.is_null() {
            // Caller passed in a metafits context, so use that
            &*metafits_context_ptr
        } else if !correlator_context_ptr.is_null() {
            // Caller passed in a correlator context, so use that
            &(*correlator_context_ptr).metafits_context
        } else {
            // Caller passed in a voltage context, so use that
            &(*voltage_context_ptr).metafits_context
        }
    };
    let mut item_vec: Vec<VisibilityPol> = Vec::new();

    // We explicitly break out the attributes so at compile time it will let us know
    // if there have been new fields added to the rust struct, then we can choose to
    // ignore them (with _) or add that field to the FFI struct.
    for item in metafits_context.visibility_pols.iter() {
        let out_item = {
            let visibility_pol::VisibilityPol { polarisation } = item;
            VisibilityPol {
                polarisation: CString::new(String::from(&*polarisation))
                    .unwrap()
                    .into_raw(),
            }
        };

        item_vec.push(out_item);
    }

    // Pass back the array and length of the array
    *out_visibility_pols_len = item_vec.len();
    *out_visibility_pols_ptr = ffi_array_to_boxed_slice(item_vec);

    // Return success
    0
}

/// Free a previously-allocated `VisibilityPol` array of structs.
///
/// # Arguments
///
/// * `visibility_pols_ptr` - pointer to an already populated `VisibilityPol` array
///
/// * `visibility_pols_len` - number of elements in the pointed to array
///
/// # Returns
///
/// * 0 on success, non-zero on failure
///
///
/// # Safety
/// * This must be called once caller is finished with the `VisibilityPol` array
/// * `visibility_pols_ptr` must point to a populated `VisibilityPol` array from the `mwalib_visibility_pols_get` function.
/// * `visibility_pols_ptr` must not have already been freed.
#[no_mangle]
pub unsafe extern "C" fn mwalib_visibility_pols_free(
    visibility_pols_ptr: *mut VisibilityPol,
    visibility_pols_len: size_t,
) -> i32 {
    // Just return 0 if the pointer is already null
    if visibility_pols_ptr.is_null() {
        return 0;
    }
    // Extract a slice from the pointer
    let slice: &mut [VisibilityPol] =
        slice::from_raw_parts_mut(visibility_pols_ptr, visibility_pols_len);
    // Now for each item we need to free anything on the heap
    for i in slice.iter_mut() {
        drop(Box::from_raw(i.polarisation));
    }

    // Free the memory for the slice
    drop(Box::from_raw(slice));

    // Return success
    0
}
