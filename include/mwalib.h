#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * The MWA's altitude in metres.
 */
#define MWA_ALTITUDE_METRES 377.827

/**
 * The MWA's latitude on Earth in radians. This is -26d42m11.94986s.
 */
#define MWA_LATITUDE_RADIANS -0.4660608448386394

/**
 * The MWA's longitude on Earth in radians. This is 116d40m14.93485s.
 */
#define MWA_LONGITUDE_RADIANS 2.0362898668561042

/**
 * Enum for all of the known variants of file format based on Correlator version
 *
 */
typedef enum {
  /**
   * MWAX correlator (v2.0)
   */
  V2,
  /**
   * MWA correlator (v1.0), having data files with "gpubox" and batch numbers in their names.
   */
  Legacy,
  /**
   * MWA correlator (v1.0), having data files without any batch numbers.
   */
  OldLegacy,
} CorrelatorVersion;

/**
 *
 * `mwalib` correlator observation context. This represents the basic metadata for a correlator observation.
 *
 */
typedef struct CorrelatorContext CorrelatorContext;

/**
 * `mwalib` metafits context. This represents the basic metadata for the observation.
 *
 */
typedef struct MetafitsContext MetafitsContext;

/**
 * Representation in C of an mwalibAntenna struct
 */
typedef struct {
  /**
   * This is the antenna number.
   * Nominally this is the field we sort by to get the desired output order of antenna.
   * X and Y have the same antenna number. This is the sorted ordinal order of the antenna.None
   * e.g. 0...N-1
   */
  uint32_t antenna;
  /**
   * Numeric part of tile_name for the antenna. Each pol has the same value
   * e.g. tile_name "tile011" hsa tile_id of 11
   */
  uint32_t tile_id;
  /**
   * Human readable name of the antenna
   * X and Y have the same name
   */
  char *tile_name;
} mwalibAntenna;

/**
 *
 * C Representation of a mwalibBaseline struct
 *
 */
typedef struct {
  /**
   * Index in the mwalibCorrelatorContext.antenna array for antenna1 for this baseline
   */
  uintptr_t antenna1_index;
  /**
   * Index in the mwalibCorrelatorContext.antenna array for antenna2 for this baseline
   */
  uintptr_t antenna2_index;
} mwalibBaseline;

/**
 * Representation in C of an mwalibCoarseChannel struct
 */
typedef struct {
  /**
   * Correlator channel is 0 indexed (0..N-1)
   */
  uintptr_t correlator_channel_number;
  /**
   * Receiver channel is 0-255 in the RRI recivers
   */
  uintptr_t receiver_channel_number;
  /**
   * gpubox channel number
   * Legacy e.g. obsid_datetime_gpuboxXX_00
   * v2     e.g. obsid_datetime_gpuboxXXX_00
   */
  uintptr_t gpubox_number;
  /**
   * Width of a coarse channel in Hz
   */
  uint32_t channel_width_hz;
  /**
   * Starting frequency of coarse channel in Hz
   */
  uint32_t channel_start_hz;
  /**
   * Centre frequency of coarse channel in Hz
   */
  uint32_t channel_centre_hz;
  /**
   * Ending frequency of coarse channel in Hz
   */
  uint32_t channel_end_hz;
} mwalibCoarseChannel;

/**
 *
 * C Representation of the `CorrelatorContext` metadata
 *
 */
typedef struct {
  /**
   * Pointer to a MetafitsContext object
   */
  MetafitsContext *metafits_context_ptr;
  /**
   * Version of the correlator format
   */
  CorrelatorVersion corr_version;
  /**
   * The proper start of the observation (the time that is common to all
   * provided gpubox files).
   */
  uint64_t start_unix_time_milliseconds;
  /**
   * `end_time_milliseconds` will is the actual end time of the observation
   * i.e. start time of last common timestep plus integration time.
   */
  uint64_t end_unix_time_milliseconds;
  /**
   * Total duration of observation (based on gpubox files)
   */
  uint64_t duration_milliseconds;
  /**
   * Number of timesteps in the observation
   */
  uintptr_t num_timesteps;
  /**
   * Number of baselines stored. This is autos plus cross correlations
   */
  uintptr_t num_baselines;
  /**
   * Number of polarisation combinations in the visibilities e.g. XX,XY,YX,YY == 4
   */
  uintptr_t num_visibility_pols;
  /**
   * Correlator mode dump time
   */
  uint64_t integration_time_milliseconds;
  /**
   * Number of coarse channels
   */
  uintptr_t num_coarse_channels;
  /**
   * Total bandwidth of observation (of the coarse channels we have)
   */
  uint32_t observation_bandwidth_hz;
  /**
   * Bandwidth of each coarse channel
   */
  uint32_t coarse_channel_width_hz;
  /**
   * Correlator fine_channel_resolution
   */
  uint32_t fine_channel_width_hz;
  /**
   * Number of fine channels in each coarse channel
   */
  uintptr_t num_fine_channels_per_coarse;
  /**
   * The number of bytes taken up by a scan/timestep in each gpubox file.
   */
  uintptr_t num_timestep_coarse_channel_bytes;
  /**
   * The number of floats in each gpubox HDU.
   */
  uintptr_t num_timestep_coarse_channel_floats;
  /**
   * This is the number of gpubox files *per batch*.
   */
  uintptr_t num_gpubox_files;
} mwalibCorrelatorMetadata;

/**
 *
 * C Representation of a mwalibTimeStep struct
 *
 */
typedef struct {
  /**
   * UNIX time (in milliseconds to avoid floating point inaccuracy)
   */
  uint64_t unix_time_ms;
} mwalibTimeStep;

/**
 *
 * C Representation of a mwalibVisibilityPol struct
 *
 */
typedef struct {
  /**
   * Polarisation (e.g. "XX" or "XY" or "YX" or "YY")
   */
  char *polarisation;
} mwalibVisibilityPol;

/**
 *
 * This a C struct to allow the caller to consume the metafits metadata
 *
 */
typedef struct {
  /**
   * Observation id
   */
  uint32_t obsid;
  /**
   * Latitude of centre point of MWA in raidans
   */
  double mwa_latitude_radians;
  /**
   * Longitude of centre point of MWA in raidans
   */
  double mwa_longitude_radians;
  /**
   * Altitude of centre poing of MWA in metres
   */
  double mwa_altitude_metres;
  /**
   * the velocity factor of electic fields in RG-6 like coax
   */
  double coax_v_factor;
  /**
   * ATTEN_DB  // global analogue attenuation, in dB
   */
  double global_analogue_attenuation_db;
  /**
   * RA tile pointing
   */
  double ra_tile_pointing_degrees;
  /**
   * DEC tile pointing
   */
  double dec_tile_pointing_degrees;
  /**
   * RA phase centre
   */
  double ra_phase_center_degrees;
  /**
   * DEC phase centre
   */
  double dec_phase_center_degrees;
  /**
   * AZIMUTH
   */
  double azimuth_degrees;
  /**
   * ALTITUDE
   */
  double altitude_degrees;
  /**
   * Altitude of Sun
   */
  double sun_altitude_degrees;
  /**
   * Distance from pointing center to Sun
   */
  double sun_distance_degrees;
  /**
   * Distance from pointing center to the Moon
   */
  double moon_distance_degrees;
  /**
   * Distance from pointing center to Jupiter
   */
  double jupiter_distance_degrees;
  /**
   * Local Sidereal Time
   */
  double lst_degrees;
  /**
   * Hour Angle of pointing center (as a string)
   */
  char *hour_angle_string;
  /**
   * GRIDNAME
   */
  char *grid_name;
  /**
   * GRIDNUM
   */
  int32_t grid_number;
  /**
   * CREATOR
   */
  char *creator;
  /**
   * PROJECT
   */
  char *project_id;
  /**
   * Observation name
   */
  char *observation_name;
  /**
   * MWA observation mode
   */
  char *mode;
  /**
   * Scheduled start (gps time) of observation
   */
  int64_t scheduled_start_utc;
  /**
   * Scheduled end (gps time) of observation
   */
  int64_t scheduled_end_utc;
  /**
   * Scheduled start (MJD) of observation
   */
  double scheduled_start_mjd;
  /**
   * Scheduled end (MJD) of observation
   */
  double scheduled_end_mjd;
  /**
   * Scheduled start (UNIX time) of observation
   */
  uint64_t scheduled_start_unix_time_milliseconds;
  /**
   * Scheduled end (UNIX time) of observation
   */
  uint64_t scheduled_end_unix_time_milliseconds;
  /**
   * Scheduled duration of observation
   */
  uint64_t scheduled_duration_milliseconds;
  /**
   * Seconds of bad data after observation starts
   */
  uint64_t quack_time_duration_milliseconds;
  /**
   * OBSID+QUACKTIM as Unix timestamp (first good timestep)
   */
  uint64_t good_time_unix_milliseconds;
  /**
   * Total number of antennas (tiles) in the array
   */
  uintptr_t num_antennas;
  /**
   * The Metafits defines an rf chain for antennas(tiles) * pol(X,Y)
   */
  uintptr_t num_rf_inputs;
  /**
   * Number of antenna pols. e.g. X and Y
   */
  uintptr_t num_antenna_pols;
  /**
   * Number of coarse channels
   */
  uintptr_t num_coarse_channels;
  /**
   * Total bandwidth of observation (of the coarse channels we have)
   */
  uint32_t observation_bandwidth_hz;
  /**
   * Bandwidth of each coarse channel
   */
  uint32_t coarse_channel_width_hz;
} mwalibMetafitsMetadata;

/**
 * Representation in C of an mwalibRFInput struct
 */
typedef struct {
  /**
   * This is the metafits order (0-n inputs)
   */
  uint32_t input;
  /**
   * This is the antenna number.
   * Nominally this is the field we sort by to get the desired output order of antenna.
   * X and Y have the same antenna number. This is the sorted ordinal order of the antenna.None
   * e.g. 0...N-1
   */
  uint32_t antenna;
  /**
   * Numeric part of tile_name for the antenna. Each pol has the same value
   * e.g. tile_name "tile011" hsa tile_id of 11
   */
  uint32_t tile_id;
  /**
   * Human readable name of the antenna
   * X and Y have the same name
   */
  char *tile_name;
  /**
   * Polarisation - X or Y
   */
  char *pol;
  /**
   * Electrical length in metres for this antenna and polarisation to the receiver
   */
  double electrical_length_m;
  /**
   * Antenna position North from the array centre (metres)
   */
  double north_m;
  /**
   * Antenna position East from the array centre (metres)
   */
  double east_m;
  /**
   * Antenna height from the array centre (metres)
   */
  double height_m;
  /**
   * AKA PFB to correlator input order (only relevant for pre V2 correlator)
   */
  uint32_t vcs_order;
  /**
   * Subfile order is the order in which this rf_input is desired in our final output of data
   */
  uint32_t subfile_order;
  /**
   * Is this rf_input flagged out (due to tile error, etc from metafits)
   */
  bool flagged;
  /**
   * Receiver number
   */
  uint32_t receiver_number;
  /**
   * Receiver slot number
   */
  uint32_t receiver_slot_number;
} mwalibRFInput;

/**
 * Free a previously-allocated `mwalibAntenna` struct.
 *
 * # Arguments
 *
 * * `antenna_ptr` - pointer to an already populated mwalibAntenna object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibAntenna object
 * * `antenna_ptr` must point to a populated mwalibAntenna object from the mwalib_antenna_get function.
 * * `antenna_ptr` must not have already been freed.
 */
void mwalib_antenna_free(mwalibAntenna *antenna_ptr);

/**
 * This returns a struct containing the requested antenna
 *
 * # Arguments
 *
 * * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object.
 *
 * * `antenna_index` - item in the antenna array to return. This must be be between 0 and context->num_antennas - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibAntenna struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `metafits_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_antenna_free` once finished, to free the rust memory.
 */
mwalibAntenna *mwalib_antenna_get(MetafitsContext *metafits_context_ptr,
                                  size_t antenna_index,
                                  uint8_t *error_message,
                                  size_t error_message_length);

/**
 * Free a previously-allocated `mwalibBaseline` struct.
 *
 * # Arguments
 *
 * * `baseline_ptr` - pointer to an already populated mwalibBaseline object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibBaseline object
 * * `baseline_ptr` must point to a populated mwalibBaseline object from the mwalib_baseline_get function.
 * * `baseline_ptr` must not have already been freed.
 */
void mwalib_baseline_free(mwalibBaseline *baseline_ptr);

/**
 * This returns a struct containing the requested baseline
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `baseline_index` - item in the baseline array to return. This must be be between 0 and context->num_baselines - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibBaseline struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_baseline_free` once finished, to free the rust memory.
 */
mwalibBaseline *mwalib_baseline_get(CorrelatorContext *correlator_context_ptr,
                                    size_t baseline_index,
                                    uint8_t *error_message,
                                    size_t error_message_length);

/**
 * Free a previously-allocated `mwalibCoarseChannel` struct.
 *
 * # Arguments
 *
 * * `coarse_channel_ptr` - pointer to an already populated mwalibCoarseChannel object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibCoarseChannel object
 * * `coarse_channel_ptr` must point to a populated mwalibCoarseChannel object from the mwalib_correlator_coarse_channel_get function.
 * * `coarse_channel_ptr` must not have already been freed.
 */
void mwalib_coarse_channel_free(mwalibCoarseChannel *coarse_channel_ptr);

/**
 * This returns a struct containing the requested correlator coarse channel
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `coarse_channel_index` - item in the coarse_channel array to return. This must be be between 0 and context->num_coarse_channels - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibCoarseChannel struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_coarse_channel_free` once finished, to free the rust memory.
 */
mwalibCoarseChannel *mwalib_correlator_coarse_channel_get(CorrelatorContext *correlator_context_ptr,
                                                          size_t coarse_channel_index,
                                                          uint8_t *error_message,
                                                          size_t error_message_length);

/**
 * Display an `CorrelatorContext` struct.
 *
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * 0 on success, 1 on failure
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must contain an `CorrelatorContext` object already populated via `mwalib_correlator_context_new`
 */
int32_t mwalib_correlator_context_display(const CorrelatorContext *correlator_context_ptr,
                                          uint8_t *error_message,
                                          size_t error_message_length);

/**
 * Free a previously-allocated `CorrelatorContext` struct (and it's members).
 *
 * # Arguments
 *
 * * `context_ptr` - pointer to an already populated `CorrelatorContext` object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the `CorrelatorContext` object
 * * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
 * * `correlator_context_ptr` must not have already been freed.
 */
void mwalib_correlator_context_free(CorrelatorContext *correlator_context_ptr);

/**
 * Free a previously-allocated float* created by `mwalib_correlator_context_read_by_baseline` or `mwalib_correlator_context_read_by_frequency`.
 *
 * Python can't free memory itself, so this is useful for Python (and perhaps
 * other languages).
 *
 * # Arguments
 *
 * * `read_buffer_ptr` - pointer to an already populated read buffer object.
 *
 * * `read_buffer_len` - length of read buffer.
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the float buffer object
 * * `read_buffer_ptr` must point to a populated float buffer from the
 *   `mwalib_correlator_context_read_by_baseline` or `mwalib_correlator_context_read_by_frequency function`.
 * * `read_buffer_ptr` must not have already been freed.
 */
void mwalib_correlator_context_free_read_buffer(float *read_buffer_ptr,
                                                const long long *read_buffer_len);

/**
 * Create and return a pointer to an `CorrelatorContext` struct based on metafits and gpubox files
 *
 * # Arguments
 *
 * * `metafits_filename` - pointer to char* buffer containing the full path and filename of a metafits file.
 *
 * * `gpubox_filenames` - pointer to array of char* buffers containing the full path and filename of the gpubox FITS files.
 *
 * * `gpubox_count` - length of the gpubox char* array.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated `CorrelatorContext` pointer or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated `char*` buffer for any error messages.
 * * Caller *must* call function `mwalib_correlator_context_free` to release the rust memory.
 */
CorrelatorContext *mwalib_correlator_context_new(const char *metafits_filename,
                                                 const char **gpubox_filenames,
                                                 size_t gpubox_count,
                                                 uint8_t *error_message,
                                                 size_t error_message_length);

/**
 * Read a single timestep / coarse channel of MWA data.
 *
 * This method takes as input a timestep_index and a coarse_channel_index to return one
 * HDU of data in [baseline][freq][pol][r][i] format
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `timestep_index` - index within the timestep array for the desired timestep. This corresponds
 *                      to mwalibTimeStep.get(context, N) where N is timestep_index.
 *
 * * `coarse_channel_index` - index within the coarse_channel array for the desired coarse channel. This corresponds
 *                            to mwalibCoarseChannel.get(context, N) where N is coarse_channel_index.
 *
 * * `buffer_ptr` - pointer to caller-owned and allocated buffer to write data into.
 *
 * * `buffer_len` - length of `buffer_ptr`.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * 0 on success, 1 on failure
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated object from the `mwalib_correlator_context_new` function.
 * * Caller *must* call `mwalib_correlator_context_free_read_buffer` function to release the rust memory.
 */
int32_t mwalib_correlator_context_read_by_baseline(CorrelatorContext *correlator_context_ptr,
                                                   uintptr_t timestep_index,
                                                   uintptr_t coarse_channel_index,
                                                   float *buffer_ptr,
                                                   size_t buffer_len,
                                                   uint8_t *error_message,
                                                   size_t error_message_length);

/**
 * Read a single timestep / coarse channel of MWA data.
 *
 * This method takes as input a timestep_index and a coarse_channel_index to return one
 * HDU of data in [freq][baseline][pol][r][i] format
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `timestep_index` - index within the timestep array for the desired timestep. This corresponds
 *                      to mwalibTimeStep.get(context, N) where N is timestep_index.
 *
 * * `coarse_channel_index` - index within the coarse_channel array for the desired coarse channel. This corresponds
 *                            to mwalibCoarseChannel.get(context, N) where N is coarse_channel_index.
 *
 * * `buffer_ptr` - pointer to caller-owned and allocated buffer to write data into.
 *
 * * `buffer_len` - length of `buffer_ptr`.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * 0 on success, 1 on failure
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated object from the `mwalib_correlator_context_new` function.
 * * Caller *must* call `mwalib_correlator_context_free_read_buffer` function to release the rust memory.
 */
int32_t mwalib_correlator_context_read_by_frequency(CorrelatorContext *correlator_context_ptr,
                                                    uintptr_t timestep_index,
                                                    uintptr_t coarse_channel_index,
                                                    float *buffer_ptr,
                                                    size_t buffer_len,
                                                    uint8_t *error_message,
                                                    size_t error_message_length);

/**
 * Free a previously-allocated `mwalibCorrelatorMetadata` struct.
 *
 * # Arguments
 *
 * * `correlator_metadata_ptr` - pointer to an already populated `mwalibCorrelatorMetadata` object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the `mwalibCorrelatorMetadata` object
 * * `correlator_metadata_ptr` must point to a populated `mwalibCorrelatorMetadata` object from the `mwalib_correlator_metadata_get` function.
 * * `correlator_metadata_ptr` must not have already been freed.
 */
void mwalib_correlator_metadata_free(mwalibCorrelatorMetadata *correlator_metadata_ptr);

/**
 * This returns a struct containing the `CorrelatorContext` metadata
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated `mwalibCorrelatorMetadata` struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated `CorrelatorContext` object from the `mwalib_correlator_context_new` function.
 * * Caller must call `mwalib_correlator_metadata_free` once finished, to free the rust memory.
 */
mwalibCorrelatorMetadata *mwalib_correlator_metadata_get(CorrelatorContext *correlator_context_ptr,
                                                         uint8_t *error_message,
                                                         size_t error_message_length);

/**
 * This returns a struct containing the requested timestep
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `timestep_index` - item in the timestep array to return. This must be be between 0 and context->num_timesteps - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibTimeStep struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_timestep_free` once finished, to free the rust memory.
 */
mwalibTimeStep *mwalib_correlator_timestep_get(CorrelatorContext *correlator_context_ptr,
                                               size_t timestep_index,
                                               uint8_t *error_message,
                                               size_t error_message_length);

/**
 * This returns a struct containing the requested visibility polarisation
 *
 * # Arguments
 *
 * * `correlator_context_ptr` - pointer to an already populated `CorrelatorContext` object.
 *
 * * `visibility_pol_index` - item in the visibility pol array to return. This must be be between 0 and context->num_visibility_pols - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibVisibilityPol struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `correlator_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_visibility_pol_free` once finished, to free the rust memory.
 */
mwalibVisibilityPol *mwalib_correlator_visibility_pol_get(CorrelatorContext *correlator_context_ptr,
                                                          size_t visibility_pol_index,
                                                          uint8_t *error_message,
                                                          size_t error_message_length);

/**
 * Free a rust-allocated CString.
 *
 * mwalib uses error strings to detail the caller with anything that went
 * wrong. Non-rust languages cannot deallocate these strings; so, call this
 * function with the pointer to do that.
 *
 * # Arguments
 *
 * * `rust_cstring` - pointer to a `char*` of a Rust string
 *
 *
 * # Returns
 *
 * * Nothing
 *
 * # Safety
 * * rust_cstring must not have already been freed and must point to a Rust string.
 */
void mwalib_free_rust_cstring(char *rust_cstring);

/**
 * Display an `MetafitsContext` struct.
 *
 *
 * # Arguments
 *
 * * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * 0 on success, 1 on failure
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `metafits_context_ptr` must contain an MetafitsContext object already populated via `mwalib_metafits_context_new`
 */
int32_t mwalib_metafits_context_display(const MetafitsContext *metafits_context_ptr,
                                        uint8_t *error_message,
                                        size_t error_message_length);

/**
 * Free a previously-allocated `MetafitsContext` struct (and it's members).
 *
 * # Arguments
 *
 * * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the `MetafitsContext` object
 * * `metafits_context_ptr` must point to a populated mwalibContext object from the `mwalib_metafits_context_new` functions.
 * * `metafits_context_ptr` must not have already been freed.
 */
void mwalib_metafits_context_free(MetafitsContext *metafits_context_ptr);

/**
 * Create and return a pointer to an `MetafitsContext` struct given only a metafits file
 *
 * # Arguments
 *
 * * `metafits_filename` - pointer to char* buffer containing the full path and filename of a metafits file.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated `MetafitsContext` pointer or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated `char*` buffer for any error messages.
 * * Caller *must* call the `mwalib_metafits_context_free` function to release the rust memory.
 */
MetafitsContext *mwalib_metafits_context_new(const char *metafits_filename,
                                             uint8_t *error_message,
                                             size_t error_message_length);

/**
 * Free a previously-allocated `mwalibMetafitsMetadata` struct.
 *
 * # Arguments
 *
 * * `metafits_metadata_ptr` - pointer to an already populated `mwalibMetafitsMetadata` object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the `mwalibMetafitsMetadata` object
 * * `metafits_metadata_ptr` must point to a populated `mwalibMetafitsMetadata` object from the `mwalib_metafits_metadata_get` function.
 * * `metafits_metadata_ptr` must not have already been freed.
 */
void mwalib_metafits_metadata_free(mwalibMetafitsMetadata *metafits_metadata_ptr);

/**
 * This returns a struct containing the `MetafitsContext` metadata
 *
 * # Arguments
 *
 * * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated `mwalibMetafitsMetadata` struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `metafits_context_ptr` must point to a populated MetafitsContext object from the `mwalib_metafits_context_new` function.
 * * Caller must call `mwalib_metafits_metadata_free` once finished, to free the rust memory.
 */
mwalibMetafitsMetadata *mwalib_metafits_metadata_get(MetafitsContext *metafits_context_ptr,
                                                     uint8_t *error_message,
                                                     size_t error_message_length);

/**
 * Free a previously-allocated `mwalibRFInput` struct.
 *
 * # Arguments
 *
 * * `rf_input_ptr` - pointer to an already populated mwalibRFInput object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibRFInput object
 * * `rf_input_ptr` must point to a populated mwalibRFInput object from the mwalib_rfinput_get function.
 * * `rf_input_ptr` must not have already been freed.
 */
void mwalib_rfinput_free(mwalibRFInput *rf_input_ptr);

/**
 * This returns a struct containing the requested antenna
 * Or NULL if there was an error
 *
 * # Arguments
 *
 * * `metafits_context_ptr` - pointer to an already populated `MetafitsContext` object.
 *
 * * `rf_input_index` - item in the rf_input array to return. This must be be between 0 and context->num_rf_inputs - 1.
 *
 * * `error_message` - pointer to already allocated buffer for any error messages to be returned to the caller.
 *
 * * `error_message_length` - length of error_message char* buffer.
 *
 *
 * # Returns
 *
 * * A Rust-owned populated mwalibRFInput struct or NULL if there was an error (check error_message)
 *
 *
 * # Safety
 * * `error_message` *must* point to an already allocated char* buffer for any error messages.
 * * `metafits_context_ptr` must point to a populated mwalibCorrelatorContext object from the mwalibCorrelatorContext_new function.
 * * Caller must call `mwalib_rfinput_free` once finished, to free the rust memory.
 */
mwalibRFInput *mwalib_rfinput_get(MetafitsContext *metafits_context_ptr,
                                  size_t rf_input_index,
                                  uint8_t *error_message,
                                  size_t error_message_length);

/**
 * Free a previously-allocated `mwalibTimeStep` struct.
 *
 * # Arguments
 *
 * * `timestep_ptr` - pointer to an already populated mwalibTimeStep object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibTimeStep object
 * * `timestep_ptr` must point to a populated mwalibTimeStep object from the mwalib_correlator_timestep_get function.
 * * `timestep_ptr` must not have already been freed.
 */
void mwalib_timestep_free(mwalibTimeStep *timestep_ptr);

/**
 * Free a previously-allocated `mwalibVisibilityPol` struct.
 *
 * # Arguments
 *
 * * `visibility_pol_ptr` - pointer to an already populated mwalibVisibilityPol object
 *
 *
 * # Returns
 *
 * * Nothing
 *
 *
 * # Safety
 * * This must be called once caller is finished with the mwalibVisibilityPol object
 * * `visibility_pol_ptr` must point to a populated mwalibVisibilityPol object from the mwalib_correlator_visibility_pol_get function.
 * * `visibility_pol_ptr` must not have already been freed.
 */
void mwalib_visibility_pol_free(mwalibVisibilityPol *visibility_pol_ptr);
