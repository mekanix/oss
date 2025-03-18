use nix::libc;
use std::fs;
use std::os::fd::AsRawFd;

const SNDCTL_DSP_MAGIC: u8 = b'P';
const SNDCTL_DSP_CHANNELS: u8 = 6;
nix::ioctl_readwrite!(oss_channels, SNDCTL_DSP_MAGIC, SNDCTL_DSP_CHANNELS, i32);

/*
typedef char oss_longname_t[64];
typedef char oss_label_t[16];
typedef char oss_devnode_t[32];

typedef struct oss_audioinfo
{
    int	dev;		/* Audio device number */
    char	name[64];
    int	busy;		/* 0, OPEN_READ, OPEN_WRITE or OPEN_READWRITE */
    int	pid;
    int	caps;		/* DSP_CAP_INPUT, DSP_CAP_OUTPUT */
    int	iformats;
    int	oformats;
    int	magic;		/* Reserved for internal use */
    char 	cmd[64];	/* Command using the device (if known) */
    int	card_number;
    int	port_number;
    int	mixer_dev;
    int	legacy_device;	/* Obsolete field. Replaced by devnode */
    int	enabled;	/* 1=enabled, 0=device not ready at this
                   moment */
    int	flags;		/* For internal use only - no practical
                   meaning */
    int	min_rate;	/* Sample rate limits */
    int	max_rate;
    int	min_channels;	/* Number of channels supported */
    int	max_channels;
    int	binding;	/* DSP_BIND_FRONT, etc. 0 means undefined */
    int	rate_source;
    char	handle[32];
    unsigned int nrates;
    unsigned int rates[20]; /* Please read the manual before using these */
    oss_longname_t	song_name;	/* Song name (if given) */
    oss_label_t	label;		/* Device label (if given) */
    int		latency;	/* In usecs, -1=unknown */
    oss_devnode_t	devnode;	/* Device special file name (inside
                       /dev) */
    int next_play_engine;
    int next_rec_engine;
    int filler[184];
} oss_audioinfo;
*/

#[repr(C)]
struct AudioInfo {
    pub dev: libc::c_int,
    pub name: [libc::c_char; 64],
    pub busy: libc::c_int,
    pub pid: libc::c_int,
    pub caps: libc::c_int,
    pub iformats: libc::c_int,
    pub oformats: libc::c_int,
    pub magic: libc::c_int,
    pub cmd: [libc::c_char; 64],
    pub card_number: libc::c_int,
    pub port_number: libc::c_int,
    pub mixer_dev: libc::c_int,
    pub legacy_device: libc::c_int,
    pub enabled: libc::c_int,
    pub flags: libc::c_int,
    pub min_rate: libc::c_int,
    pub max_rate: libc::c_int,
    pub min_channels: libc::c_int,
    pub max_channels: libc::c_int,
    pub binding: libc::c_int,
    pub rate_source: libc::c_int,
    pub handle: [libc::c_char; 32],
    pub nrates: libc::c_uint,
    pub rates: [libc::c_uint; 20],
    pub song_name: [libc::c_char; 64],
    pub label: [libc::c_char; 16],
    pub latency: libc::c_int,
    pub devnode: [libc::c_char; 32],
    pub next_play_engine: libc::c_int,
    pub next_rec_engine: libc::c_int,
    pub filler: [libc::c_int; 184],
}

impl AudioInfo {
    fn new() -> AudioInfo {
        AudioInfo {
            dev: 0,
            name: [0; 64],
            busy: 0,
            pid: 0,
            caps: 0,
            iformats: 0,
            oformats: 0,
            magic: 0,
            cmd: [0; 64],
            card_number: 0,
            port_number: 0,
            mixer_dev: 0,
            legacy_device: 0,
            enabled: 0,
            flags: 0,
            min_rate: 0,
            max_rate: 0,
            min_channels: 0,
            max_channels: 0,
            binding: 0,
            rate_source: 0,
            handle: [0; 32],
            nrates: 0,
            rates: [0; 20],
            song_name: [0; 64],
            label: [0; 16],
            latency: 0,
            devnode: [0; 32],
            next_play_engine: 0,
            next_rec_engine: 0,
            filler: [0; 184],
        }
    }
}

const SNDCTL_INFO_MAGIC: u8 = b'X';
const SNDCTL_ENGINEINFO: u8 = 12;
nix::ioctl_readwrite!(
    oss_audio_info,
    SNDCTL_INFO_MAGIC,
    SNDCTL_ENGINEINFO,
    AudioInfo
);

fn main() {
    let devpath = String::from("/dev/dsp");
    let dsp = fs::File::open(devpath).unwrap();
    let fd = dsp.as_raw_fd();
    let mut channels: i32 = 2;
    let mut audio_info = AudioInfo::new();
    unsafe {
        oss_channels(fd, &mut channels).expect("Failed to set number of channels");
        oss_audio_info(fd, &mut audio_info).expect("Failed to get info on device");
    }
    println!("channels = {}", audio_info.max_channels);
    println!("rate = {}", audio_info.max_rate);
}
