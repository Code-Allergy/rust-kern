#ifndef FAT32_H
#define FAT32_H

#include <stdint.h>
#include <stddef.h>

#define MBR_SIGNATURE 0xAA55

#define FAT32_ERROR_IO                  (-1)
#define FAT32_ERROR_INVALID_BOOT_SECTOR (-2)
#define FAT32_ERROR_NO_FILE             (-3)
#define FAT32_ERROR_BAD_PARAMETER       (-4)
#define FAT32_ERROR_CORRUPTED_FS        (-5)
#define FAT32_ERROR_NO_SPACE            (-6)
#define FAT32_ERROR_NO_DIR              (-7)
#define FAT32_ERROR_NO_PATH             (-8)
#define FAT32_ERROR_IS_DIR              (-9)
#define FAT32_ERROR_NOT_DIR             (-10)
#define FAT32_ERROR_END_OF_DIR          (-11)
#define FAT32_ERROR_NO_LABEL            (-12)
#define FAT32_ERROR_INVALID_CLUSTER     (-13)

#define FAT32_SUCCESS 0
#define FAT32_EOC     1

#define FAT32_MAGIC                 0xF32F32F3
#define FAT32_SECTOR_SIZE           512
#define FAT32_BOOT_SECTOR           0
#define FAT32_BOOT_SECTOR_SIGNATURE 0xAA55
#define FAT32_MAX_PATH              256
#define FAT32_MAX_COMPONENTS        16
#define FAT32_MAX_COMPONENT_LENGTH  64
#define FAT32_PART_TYPE_1           0x0B
#define FAT32_PART_TYPE_2           0x0C

#define FAT32_LABEL_LEN 12

#define FAT32_ATTR_DIRECTORY 0x10
#define FAT32_ATTR_ARCHIVE   0x20
#define FAT32_ATTR_VOLLABEL  0x08
#define FAT32_ATTR_HIDDEN    0x02
#define FAT32_ATTR_SYSTEM    0x04
#define FAT32_ATTR_RO        0x01
#define FAT32_ATTR_LFN       0x0F

#define FAT32_EOC_MARKER  0x0FFFFFFF
#define FAT32_LAST_MARKER 0x0FFFFFF8

#define FAT32_ENTRY_EMPTY   0x00
#define FAT32_ENTRY_DELETED 0xE5
#define FAT32_ENTRY_DOT     0x2E

/* static only in bootloader, we can make a dynamic version later */
typedef struct
{
    char components[FAT32_MAX_COMPONENTS][FAT32_MAX_COMPONENT_LENGTH];
    int num_components;
} fat32_path_static_t;

typedef fat32_path_static_t fat32_path_t;

static inline void fat32_path_init(fat32_path_t* path) { path->num_components = 0; }

/* api structs */

/*
 * Disk I/O abstraction.
 */
typedef struct
{
    /*
     * read one sector from somewhere.
     *   sector:   The absolute sector number to read.
     *   buffer:   A pointer to a buffer with at least 'bytes_per_sector' bytes.
     * Returns 0 on success.
     */
    int (*read_sector)(uint32_t sector, uint8_t* buffer);
} fat32_diskio_t;

/*
 *  FS abstraction
 */
typedef struct
{
    fat32_diskio_t disk;

    uint16_t bytes_per_sector;
    uint8_t sectors_per_cluster;
    uint16_t reserved_sector_count;
    uint8_t num_fats;
    uint32_t sectors_per_fat; /* For FAT32, BPB_FATSz32 */
    uint32_t root_cluster;    /* For FAT32, BPB_RootClus */
    uint32_t cluster_size;    /* Sectors per cluster * bytes per sector */
    uint32_t total_clusters;
    uint32_t total_sectors;
    uint32_t fsinfo_sector;

    uint32_t first_data_sector; /* The first sector of the data region */
    uint32_t fat_start_sector;  /* The first FAT's starting sector */
    uint32_t magic;             /* Flag indicating if the fs struct is initialized */
} fat32_fs_t;

static inline int fat32_is_initialized(const fat32_fs_t* fs) { return fs && fs->magic == FAT32_MAGIC; }

/*
 * File handle structure.
 *
 * Used to track a file opened from the FAT32 volume.
 */
typedef struct
{
    fat32_fs_t* fs;                 /* Reference to the mounted filesystem */
    char formatted_name[11];        /* 8.3 formatted name */
    uint32_t start_cluster;         /* Starting cluster number from the directory entry */
    uint32_t current_cluster;       /* Current cluster number while reading */
    uint32_t current_cluster_index; /* Index of last cluster */
    uint32_t file_size;             /* Total file size in bytes */
    uint32_t file_offset;           /* Current offset into the file */
    uint32_t parent_dir_cluster;    /* Cluster of the directory containing this file */
} fat32_file_t;

typedef struct
{
    uint32_t start_cluster;   /* Starting cluster of the directory */
    uint32_t current_cluster; /* Current cluster in iteration */
    uint32_t current_sector;  /* Current sector within cluster */
    uint32_t current_entry;   /* Current entry within sector */
    fat32_fs_t* fs;           /* Reference to filesystem */
} fat32_dir_t;

/*
 * Directory entry structure.
 *
 * A minimal representation of a directory entry (only the short filename variant).
 */
typedef struct
{
    char name[256];         /* LFN */
    uint8_t attributes;     /* File attributes */
    uint32_t file_size;     /* File size in bytes */
    uint32_t start_cluster; /* Starting cluster number */
    uint32_t is_initialized;
} fat32_dir_entry_t;

/*===========================================================================
  API Functions
  These functions comprise the public interface to the FAT32 driver.
  ===========================================================================*/

/* FS functions */
/**
 * @brief Mounts (initializes) the FAT32 filesystem.
 *
 * This function reads the boot sector and sets up the FAT32 filesystem structure.
 *
 * @param fs       Pointer to a fat32_fs_t structure (allocated by the caller).
 * @param io       Pointer to a fat32_diskio_t structure with I/O function pointers.
 * @return         FAT32_SUCCESS on success, or an error code.
 */
int fat32_mount(fat32_fs_t* fs, const fat32_diskio_t* io);

/**
 * @brief Reads the label of the mounted FAT32 filesystem.
 *
 * Reads the volume label from the root directory of the FAT32 filesystem.
 *
 * @param fs         Mounted FAT32 filesystem pointer.
 * @param label_out  Pointer to a buffer to store the label.
 *                   Should be at least FAT32_LABEL_LEN bytes.
 */
int fat32_readlabel(fat32_fs_t* fs, char* label_out);

/* File ops */
/**
 * @brief Opens a file given its path.
 *
 * Given a path (e.g., "/boot/kernel.bin" for a file in the root directory),
 * this function locates the file and fills in a fat32_file_t structure.
 *
 * @param fs       Mounted FAT32 filesystem pointer.
 * @param path     Null-terminated path to the file.
 * @param file     Pointer to a fat32_file_t structure (allocated by the caller).
 * @return         FAT32_SUCCESS on success, or an error code.
 */
int fat32_open(fat32_fs_t* fs, const char* path, fat32_file_t* file);

/**
 * @brief Reads data from an open file.
 *
 * Reads up to 'size' bytes from the file, updating the file offset
 * and following cluster chains as necessary.
 *
 * @param file         Pointer to an open fat32_file_t.
 * @param buffer       Pointer to the destination buffer.
 * @param size         Number of bytes to read.
 * @return             Real bytes read on success, or an error code.
 */
int fat32_read(fat32_file_t* file, void* buffer, int size);

int fat32_tell(fat32_file_t* file);
int fat32_seek(fat32_file_t* file, int offset);
int fat32_eof(fat32_file_t* file);
int fat32_size(fat32_file_t* file);

/**
 * @brief Closes an open file.
 *
 * Currently, this may simply be a placeholder as no cleanup is needed.
 *
 * @param file     Pointer to the fat32_file_t to close.
 * @return         FAT32_SUCCESS on success, or an error code.
 */
int fat32_close(fat32_file_t* file);

/*                                */
/* parsing structs, do not modify */
/*                                */
typedef struct __attribute__((packed))
{
    uint8_t jmpBoot[3];        /* Jump instruction */
    char oemName[8];           /* OEM Name */
    uint16_t bytesPerSector;   /* Bytes per sector (typically 512) */
    uint8_t sectorsPerCluster; /* Sectors per cluster */
    uint16_t reservedSectors;  /* Reserved sectors (Boot Sector + FSInfo) */
    uint8_t numFATs;           /* Number of FAT tables (usually 2) */
    uint16_t rootEntryCount;   /* Always 0 for FAT32 */
    uint16_t totalSectors16;   /* Only used if < 65536 sectors, else 0 */
    uint8_t mediaType;         /* Media descriptor */
    uint16_t sectorsPerFAT16;  /* Unused in FAT32 */
    uint16_t sectorsPerTrack;  /* Geometry data */
    uint16_t numHeads;         /* Geometry data */
    uint32_t hiddenSectors;    /* Used for partitions (0 in superfloppy) */
    uint32_t totalSectors32;   /* Total sectors for FAT32 */
    uint32_t sectorsPerFAT32;  /* Sectors per FAT */
    uint16_t extFlags;         /* Mirroring info */
    uint16_t fsVersion;        /* FAT32 version */
    uint32_t rootCluster;      /* First cluster of root directory (typically 2) */
    uint16_t fsInfo;           /* Sector number of FSInfo structure */
    uint16_t backupBootSector; /* Sector number of backup boot sector */
    uint8_t reserved[12];      /* Unused */
    uint8_t driveNumber;       /* BIOS drive number */
    uint8_t reserved1;         /* Unused */
    uint8_t bootSignature;     /* 0x29 if extended boot signature */
    uint32_t volumeID;         /* Volume serial number */
    char volumeLabel[11];      /* Volume label */
    char fsType[8];            /* "FAT32   " */
    uint8_t bootCode[420];     /* Bootloader code */
    uint16_t bootSectorSig;    /* 0xAA55 (Boot sector signature) */
} Fat32BootSector;

typedef struct __attribute__((packed))
{
    uint8_t filename[11];      /* File name (padded with spaces) */
    uint8_t attr;              /* Attributes (readonly, hidden, system, etc.) */
    uint8_t ntRes;             /* Reserved for Windows NT */
    uint8_t creationTimeTenth; /* Tenth-of-a-second timestamp */
    uint16_t creationTime;     /* Creation time */
    uint16_t creationDate;     /* Creation date */
    uint16_t lastAccessDate;   /* Last access date */
    uint16_t firstClusterHigh; /* High word of first cluster number */
    uint16_t writeTime;        /* Last modified time */
    uint16_t writeDate;        /* Last modified date */
    uint16_t firstClusterLow;  /* Low word of first cluster number */
    uint32_t fileSize;         /* File size in bytes */
} Fat32DirectoryEntry;

/* TODO - LFN entries */
typedef struct __attribute__((packed))
{
    uint8_t sequence_number;
    uint16_t name1[5]; /* First 5 characters (UTF-16LE) */
    uint8_t attr;      /* Always 0x0F for LFN */
    uint8_t type;      /* Zero for LFN */
    uint8_t checksum;
    uint16_t name2[6]; /* Next 6 characters */
    uint16_t cluster;  /* Zero for LFN */
    uint16_t name3[2]; /* Final 2 characters */
} Fat32LFNDirectoryEntry;

typedef struct __attribute__((packed))
{
    uint8_t boot_flag;
    uint8_t start_chs[3];
    uint8_t partition_type;
    uint8_t end_chs[3];
    uint32_t start_sector;
    uint32_t total_sectors;
} MBRPartitionEntry;

typedef struct __attribute__((packed))
{
    uint8_t bootstrap[446];
    MBRPartitionEntry partitions[4];
    uint16_t signature;
} MBR;

#endif /* FAT32_H */
