pub const NUM_QUEUES: i32 = 4;
pub const QUEUE_LENGTH: i32 = 10;
pub const NUM_PROCESSORS: i32 = 4;
pub const NUM_RESOURCES: i32 = 8;

pub enum JobType {
	INTERACTIVE,
    REALTIME,
    BATCH,
    UNKNOWN,
    NUM_TYPES,
}