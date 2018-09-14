/// This is the main user configuration
/// This struct should contain all user customizable options as this is passed to the channel to be accessed
#[derive(Copy, Clone)] 
pub struct UserConfigurations{
	/// optional user spesefied channel limits
	/// These should stay the same for a channel and cannot change during the life of a channel
    pub channel_limits : ChannelLimits,
	/// Channel options
	pub channel_options : ChannelOptions,
}

impl UserConfigurations {
    pub fn new() -> Self{
        UserConfigurations {
            channel_limits : ChannelLimits::new(),
			channel_options : ChannelOptions::new(),
        }
    }
}

/// This struct contains all the optional bolt 2 channel limits.
/// If the user wants to check a value, the value needs to be filled in, as by default they are not checked
#[derive(Copy, Clone)]
pub struct ChannelLimits{
	/// minimum allowed funding_satoshis
	pub funding_satoshis :u64,
	/// maximum allowed htlc_minimum_msat
	pub htlc_minimum_msat : u64,
	/// min allowed max_htlc_value_in_flight_msat
	pub max_htlc_value_in_flight_msat : u64,
	/// max allowed channel_reserve_satashis
	pub channel_reserve_satoshis : u64,
	/// min allowed max_accepted_htlcs
	pub max_accepted_htlcs : u16,
	/// min allowed dust_limit_satashis
	pub dust_limit_satoshis : u64,
}

impl ChannelLimits {
//creating max and min possible values because if they are not set, means we should not check them.
	pub fn new() -> Self{
		ChannelLimits {
			funding_satoshis : 0,
			htlc_minimum_msat : <u64>::max_value(),
			max_htlc_value_in_flight_msat : 0,
			channel_reserve_satoshis : <u64>::max_value(),
			max_accepted_htlcs : 0,
			dust_limit_satoshis : 0,
		}
	}
}

/// This struct contains all the custom channel options.
#[derive(Copy, Clone)]
pub struct ChannelOptions{
	/// Amount (in millionths of a satoshi) channel will charge per transferred satoshi.
	pub fee_proportional_millionths : u32, 	
	///should the channel be annouced or not
	pub annouce_channel : bool,
}
impl ChannelOptions {
	/// creating a struct with values. 
	/// fee_proportional_millionths should be changed afterwords 
	pub fn new() -> Self{
		ChannelOptions {
			fee_proportional_millionths : 0,
			annouce_channel : true,
		}
	}
}