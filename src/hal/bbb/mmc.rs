use crate::hal::{
    asm,
    util::{reg32_clear_bits, reg32_read, reg32_read_masked, reg32_write, reg32_write_masked},
};

// BASE
pub const MMC0_BASE: u32 = 0x48060000;

// REGISTERS
pub const MMC_HL_REV: u32 = 0x0;
pub const MMC_HL_HWINFO: u32 = 0x4;
pub const MMC_HL_SYSCONFIG: u32 = 0x10;
pub const MMC_SYSCONFIG: u32 = 0x110;
pub const MMC_SYSSTATUS: u32 = 0x114;
pub const MMC_CSRE: u32 = 0x124;
pub const MMC_SYSTEST: u32 = 0x128;
pub const MMC_CON: u32 = 0x12C;
pub const MMC_PWCNT: u32 = 0x130;
pub const MMC_SDMASA: u32 = 0x200;
pub const MMC_BLK: u32 = 0x204;
pub const MMC_ARG: u32 = 0x208;
pub const MMC_CMD: u32 = 0x20C;
pub const MMC_RSP10: u32 = 0x210;
pub const MMC_RSP32: u32 = 0x214;
pub const MMC_RSP54: u32 = 0x218;
pub const MMC_RSP76: u32 = 0x21C;
pub const MMC_DATA: u32 = 0x220;
pub const MMC_PSTATE: u32 = 0x224;
pub const MMC_HCTL: u32 = 0x228;
pub const MMC_SYSCTL: u32 = 0x22C;
pub const MMC_STAT: u32 = 0x230;
pub const MMC_IE: u32 = 0x234;
pub const MMC_ISE: u32 = 0x238;
pub const MMC_AC12: u32 = 0x23C;
pub const MMC_CAPA: u32 = 0x240;
pub const MMC_CUR_CAPA: u32 = 0x248;
pub const MMC_FE: u32 = 0x250;
pub const MMC_ADMAES: u32 = 0x254;
pub const MMC_ADMASAL: u32 = 0x258;
pub const MMC_REV: u32 = 0x2FC;

// fields
/* SYSCONFIG */
pub const MMC_SYSCONFIG_AUTOIDLE: u32 = 0x00000001;
pub const MMC_SYSCONFIG_AUTOIDLE_SHIFT: u32 = 0x00000000;
pub const MMC_SYSCONFIG_AUTOIDLE_OFF: u32 = 0x0;
pub const MMC_SYSCONFIG_AUTOIDLE_ON: u32 = 0x1;

pub const MMC_SYSCONFIG_CLOCKACTIVITY: u32 = 0x00000300;
pub const MMC_SYSCONFIG_CLOCKACTIVITY_SHIFT: u32 = 0x00000008;
pub const MMC_SYSCONFIG_CLOCKACTIVITY_BOTH: u32 = 0x3;
pub const MMC_SYSCONFIG_CLOCKACTIVITY_FUNC: u32 = 0x2;
pub const MMC_SYSCONFIG_CLOCKACTIVITY_NONE: u32 = 0x0;
pub const MMC_SYSCONFIG_CLOCKACTIVITY_OCP: u32 = 0x1;

pub const MMC_SYSCONFIG_ENAWAKEUP: u32 = 0x00000004;
pub const MMC_SYSCONFIG_ENAWAKEUP_SHIFT: u32 = 0x00000002;
pub const MMC_SYSCONFIG_ENAWAKEUP_DISABLED: u32 = 0x0;
pub const MMC_SYSCONFIG_ENAWAKEUP_ENABLED: u32 = 0x1;

pub const MMC_SYSCONFIG_SIDLEMODE: u32 = 0x00000018;
pub const MMC_SYSCONFIG_SIDLEMODE_SHIFT: u32 = 0x00000003;
pub const MMC_SYSCONFIG_SIDLEMODE_FORCE: u32 = 0x0;
pub const MMC_SYSCONFIG_SIDLEMODE_NOIDLE: u32 = 0x1;
pub const MMC_SYSCONFIG_SIDLEMODE_SMART: u32 = 0x2;
pub const MMC_SYSCONFIG_SIDLEMODE_SMARTWAKE: u32 = 0x3;

pub const MMC_SYSCONFIG_SOFTRESET: u32 = 0x00000002;
pub const MMC_SYSCONFIG_SOFTRESET_SHIFT: u32 = 0x00000001;
pub const MMC_SYSCONFIG_SOFTRESET_NORESET: u32 = 0x0;
pub const MMC_SYSCONFIG_SOFTRESET_ONRESET: u32 = 0x1;
pub const MMC_SYSCONFIG_SOFTRESET_ST_RST: u32 = 0x1;
pub const MMC_SYSCONFIG_SOFTRESET_ST_UN: u32 = 0x0;

pub const MMC_SYSCONFIG_STANDBYMODE: u32 = 0x00003000;
pub const MMC_SYSCONFIG_STANDBYMODE_SHIFT: u32 = 0x0000000C;
pub const MMC_SYSCONFIG_STANDBYMODE_FORCE: u32 = 0x0;
pub const MMC_SYSCONFIG_STANDBYMODE_NOIDLE: u32 = 0x1;
pub const MMC_SYSCONFIG_STANDBYMODE_SMART: u32 = 0x2;
pub const MMC_SYSCONFIG_STANDBYMODE_SMARTWAKE: u32 = 0x3;

/* SYSCTL */
pub const MMC_SYSCTL_CEN: u32 = 0x00000004;
pub const MMC_SYSCTL_CEN_SHIFT: u32 = 0x00000002;
pub const MMC_SYSCTL_CEN_DISABLE: u32 = 0x0;
pub const MMC_SYSCTL_CEN_ENABLE: u32 = 0x1;

pub const MMC_SYSCTL_CLKD: u32 = 0x0000FFC0;
pub const MMC_SYSCTL_CLKD_SHIFT: u32 = 0x00000006;
pub const MMC_SYSCTL_CLKD_BYPASS0: u32 = 0x0;
pub const MMC_SYSCTL_CLKD_BYPASS1: u32 = 0x1;
pub const MMC_SYSCTL_CLKD_DIV1023: u32 = 0x3FF;
pub const MMC_SYSCTL_CLKD_DIV2: u32 = 0x2;
pub const MMC_SYSCTL_CLKD_DIV3: u32 = 0x3;

pub const MMC_SYSCTL_DTO: u32 = 0x000F0000;
pub const MMC_SYSCTL_DTO_SHIFT: u32 = 0x00000010;
pub const MMC_SYSCTL_DTO_15THDTO: u32 = 0xE;
pub const MMC_SYSCTL_DTO_1STDTO: u32 = 0x0;
pub const MMC_SYSCTL_DTO_2NDDTO: u32 = 0x1;
pub const MMC_SYSCTL_DTO_RSVD: u32 = 0xF;

pub const MMC_SYSCTL_ICE: u32 = 0x00000001;
pub const MMC_SYSCTL_ICE_SHIFT: u32 = 0x00000000;
pub const MMC_SYSCTL_ICE_OSCILLATE: u32 = 0x1;
pub const MMC_SYSCTL_ICE_STOP: u32 = 0x0;

pub const MMC_SYSCTL_ICS: u32 = 0x00000002;
pub const MMC_SYSCTL_ICS_SHIFT: u32 = 0x00000001;
pub const MMC_SYSCTL_ICS_NOTREADY: u32 = 0x0;
pub const MMC_SYSCTL_ICS_READY: u32 = 0x1;

pub const MMC_SYSCTL_SRA: u32 = 0x01000000;
pub const MMC_SYSCTL_SRA_SHIFT: u32 = 0x00000018;
pub const MMC_SYSCTL_SRA_RSTCOMPLETED: u32 = 0x0;
pub const MMC_SYSCTL_SRA_RSTFORALLDESIGN: u32 = 0x1;

pub const MMC_SYSCTL_SRC: u32 = 0x02000000;
pub const MMC_SYSCTL_SRC_SHIFT: u32 = 0x00000019;
pub const MMC_SYSCTL_SRC_RSTCOMPLETED: u32 = 0x0;
pub const MMC_SYSCTL_SRC_RSTFORCMD: u32 = 0x1;

pub const MMC_SYSCTL_SRD: u32 = 0x04000000;
pub const MMC_SYSCTL_SRD_SHIFT: u32 = 0x0000001A;
pub const MMC_SYSCTL_SRD_RSTCOMPLETED: u32 = 0x0;
pub const MMC_SYSCTL_SRD_RSTFORDAT: u32 = 0x1;

/* SYSSTATUS */
pub const MMC_SYSSTATUS_RESETDONE: u32 = 0x00000001;
pub const MMC_SYSSTATUS_RESETDONE_SHIFT: u32 = 0x00000000;
pub const MMC_SYSSTATUS_RESETDONE_DONE: u32 = 0x1;
pub const MMC_SYSSTATUS_RESETDONE_ONGOING: u32 = 0x0;

/* CAPA */
pub const MMC_CAPA_AD2S: u32 = 0x00080000;
pub const MMC_CAPA_AD2S_SHIFT: u32 = 0x00000013;
pub const MMC_CAPA_AD2S_ADMA2NOTSUPPORTED: u32 = 0x0;
pub const MMC_CAPA_AD2S_ADMA2SUPPORTED: u32 = 0x1;

pub const MMC_CAPA_BCF: u32 = 0x00003F00;
pub const MMC_CAPA_BCF_SHIFT: u32 = 0x00000008;
pub const MMC_CAPA_BCF_OMETH: u32 = 0x0;

pub const MMC_CAPA_BIT64: u32 = 0x10000000;
pub const MMC_CAPA_BIT64_SHIFT: u32 = 0x0000001C;
pub const MMC_CAPA_BIT64_SYSADDR32B: u32 = 0x0;
pub const MMC_CAPA_BIT64_SYSADDR64B: u32 = 0x1;

pub const MMC_CAPA_DS: u32 = 0x00400000;
pub const MMC_CAPA_DS_SHIFT: u32 = 0x00000016;
pub const MMC_CAPA_DS_NOTSUPPORTED: u32 = 0x0;
pub const MMC_CAPA_DS_SUPPORTED: u32 = 0x1;

pub const MMC_CAPA_HSS: u32 = 0x00200000;
pub const MMC_CAPA_HSS_SHIFT: u32 = 0x00000015;
pub const MMC_CAPA_HSS_NOTSUPPORTED: u32 = 0x0;
pub const MMC_CAPA_HSS_SUPPORTED: u32 = 0x1;

pub const MMC_CAPA_MBL: u32 = 0x00030000;
pub const MMC_CAPA_MBL_SHIFT: u32 = 0x00000010;
pub const MMC_CAPA_MBL_1024: u32 = 0x1;
pub const MMC_CAPA_MBL_2048: u32 = 0x2;
pub const MMC_CAPA_MBL_512: u32 = 0x0;

pub const MMC_CAPA_SRS: u32 = 0x00800000;
pub const MMC_CAPA_SRS_SHIFT: u32 = 0x00000017;
pub const MMC_CAPA_SRS_NOTSUPPORTED: u32 = 0x0;
pub const MMC_CAPA_SRS_SUPPORTED: u32 = 0x1;

pub const MMC_CAPA_TCF: u32 = 0x0000003F;
pub const MMC_CAPA_TCF_SHIFT: u32 = 0x00000000;
pub const MMC_CAPA_TCF_OMETH: u32 = 0x0;

pub const MMC_CAPA_TCU: u32 = 0x00000080;
pub const MMC_CAPA_TCU_SHIFT: u32 = 0x00000007;
pub const MMC_CAPA_TCU_KHZ: u32 = 0x0;
pub const MMC_CAPA_TCU_MHZ: u32 = 0x1;

pub const MMC_CAPA_VS18: u32 = 0x04000000;
pub const MMC_CAPA_VS18_SHIFT: u32 = 0x0000001A;
pub const MMC_CAPA_VS18_1V8_NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS18_1V8_SUP: u32 = 0x1;
pub const MMC_CAPA_VS18_ST_1V8NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS18_ST_1V8SUP: u32 = 0x1;

pub const MMC_CAPA_VS30: u32 = 0x02000000;
pub const MMC_CAPA_VS30_SHIFT: u32 = 0x00000019;
pub const MMC_CAPA_VS30_3V0_NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS30_3V0_SUP: u32 = 0x1;
pub const MMC_CAPA_VS30_ST_3V0NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS30_ST_3V0SUP: u32 = 0x1;

pub const MMC_CAPA_VS33: u32 = 0x01000000;
pub const MMC_CAPA_VS33_SHIFT: u32 = 0x00000018;
pub const MMC_CAPA_VS33_3V3_NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS33_3V3_SUP: u32 = 0x1;
pub const MMC_CAPA_VS33_ST_3V3NOTSUP: u32 = 0x0;
pub const MMC_CAPA_VS33_ST_3V3SUP: u32 = 0x1;

pub const MMCHS_HCTL_SDBP: u32 = 0x00000100;
pub const MMCHS_HCTL_SDBP_SHIFT: u32 = 0x00000008;
pub const MMCHS_HCTL_SDBP_PWROFF: u32 = 0x0;
pub const MMCHS_HCTL_SDBP_PWRON: u32 = 0x1;

pub const MMCHS_HCTL_SDVS: u32 = 0x00000E00;
pub const MMCHS_HCTL_SDVS_SHIFT: u32 = 0x00000009;
pub const MMCHS_HCTL_SDVS_1V8: u32 = 0x5;
pub const MMCHS_HCTL_SDVS_3V0: u32 = 0x6;
pub const MMCHS_HCTL_SDVS_3V3: u32 = 0x7;

pub const MMCHS_HCTL_HSPE: u32 = 0x00000004;
pub const MMCHS_HCTL_HSPE_SHIFT: u32 = 0x00000002;
pub const MMCHS_HCTL_HSPE_HIGHSPEED: u32 = 0x1;
pub const MMCHS_HCTL_HSPE_NORMALSPEED: u32 = 0x0;

// freqs

pub const MMCSD_IN_FREQ: u32 = 96000000; /* 96MHz */
pub const MMCSD_INIT_FREQ: u32 = 400000; /* 400kHz */

// sigen
pub const HS_MMCSD_SIGEN_BADACCESS: u32 = MMCHS_ISE_BADA_SIGEN;
pub const HS_MMCSD_SIGEN_CARDERROR: u32 = MMCHS_ISE_CERR_SIGEN;
pub const HS_MMCSD_SIGEN_ADMAERROR: u32 = MMCHS_ISE_ADMAE_SIGEN;
pub const HS_MMCSD_SIGEN_ACMD12ERR: u32 = MMCHS_ISE_ACE_SIGEN;
pub const HS_MMCSD_SIGEN_DATABITERR: u32 = MMCHS_ISE_DEB_SIGEN;
pub const HS_MMCSD_SIGEN_DATACRCERR: u32 = MMCHS_ISE_DCRC_SIGEN;
pub const HS_MMCSD_SIGEN_DATATIMEOUT: u32 = MMCHS_ISE_DTO_SIGEN;
pub const HS_MMCSD_SIGEN_CMDINDXERR: u32 = MMCHS_ISE_CIE_SIGEN;
pub const HS_MMCSD_SIGEN_CMDBITERR: u32 = MMCHS_ISE_CEB_SIGEN;
pub const HS_MMCSD_SIGEN_CMDCRCERR: u32 = MMCHS_ISE_CCRC_SIGEN;
pub const HS_MMCSD_SIGEN_CMDTIMEOUT: u32 = MMCHS_ISE_CTO_SIGEN;
pub const HS_MMCSD_SIGEN_CARDINS: u32 = MMCHS_ISE_CINS_SIGEN;
pub const HS_MMCSD_SIGEN_BUFRDRDY: u32 = MMCHS_ISE_BRR_SIGEN;
pub const HS_MMCSD_SIGEN_BUFWRRDY: u32 = MMCHS_ISE_BWR_SIGEN;
pub const HS_MMCSD_SIGEN_TRNFCOMP: u32 = MMCHS_ISE_TC_SIGEN;
pub const HS_MMCSD_SIGEN_CMDCOMP: u32 = MMCHS_ISE_CC_SIGEN;

// ISE
pub const MMCHS_ISE_ACE_SIGEN: u32 = 0x01000000;
pub const MMCHS_ISE_ACE_SIGEN_SHIFT: u32 = 0x00000018;
pub const MMCHS_ISE_ACE_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_ACE_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_ADMAE_SIGEN: u32 = 0x02000000;
pub const MMCHS_ISE_ADMAE_SIGEN_SHIFT: u32 = 0x00000019;
pub const MMCHS_ISE_ADMAE_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_ADMAE_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_BADA_SIGEN: u32 = 0x20000000;
pub const MMCHS_ISE_BADA_SIGEN_SHIFT: u32 = 0x0000001D;
pub const MMCHS_ISE_BADA_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_BADA_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_BGE_SIGEN: u32 = 0x00000004;
pub const MMCHS_ISE_BGE_SIGEN_SHIFT: u32 = 0x00000002;
pub const MMCHS_ISE_BGE_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_BGE_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_BRR_SIGEN: u32 = 0x00000020;
pub const MMCHS_ISE_BRR_SIGEN_SHIFT: u32 = 0x00000005;
pub const MMCHS_ISE_BRR_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_BRR_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_BSR_SIGEN: u32 = 0x00000400;
pub const MMCHS_ISE_BSR_SIGEN_SHIFT: u32 = 0x0000000A;
pub const MMCHS_ISE_BSR_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_BSR_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_BWR_SIGEN: u32 = 0x00000010;
pub const MMCHS_ISE_BWR_SIGEN_SHIFT: u32 = 0x00000004;
pub const MMCHS_ISE_BWR_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_BWR_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CCRC_SIGEN: u32 = 0x00020000;
pub const MMCHS_ISE_CCRC_SIGEN_SHIFT: u32 = 0x00000011;
pub const MMCHS_ISE_CCRC_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CCRC_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CC_SIGEN: u32 = 0x00000001;
pub const MMCHS_ISE_CC_SIGEN_SHIFT: u32 = 0x00000000;
pub const MMCHS_ISE_CC_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CC_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CEB_SIGEN: u32 = 0x00040000;
pub const MMCHS_ISE_CEB_SIGEN_SHIFT: u32 = 0x00000012;
pub const MMCHS_ISE_CEB_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CEB_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CERR_SIGEN: u32 = 0x10000000;
pub const MMCHS_ISE_CERR_SIGEN_SHIFT: u32 = 0x0000001C;
pub const MMCHS_ISE_CERR_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CERR_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CIE_SIGEN: u32 = 0x00080000;
pub const MMCHS_ISE_CIE_SIGEN_SHIFT: u32 = 0x00000013;
pub const MMCHS_ISE_CIE_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CIE_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CINS_SIGEN: u32 = 0x00000040;
pub const MMCHS_ISE_CINS_SIGEN_SHIFT: u32 = 0x00000006;
pub const MMCHS_ISE_CINS_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CINS_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CIRQ_SIGEN: u32 = 0x00000100;
pub const MMCHS_ISE_CIRQ_SIGEN_SHIFT: u32 = 0x00000008;
pub const MMCHS_ISE_CIRQ_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CIRQ_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CREM_SIGEN: u32 = 0x00000080;
pub const MMCHS_ISE_CREM_SIGEN_SHIFT: u32 = 0x00000007;
pub const MMCHS_ISE_CREM_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CREM_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_CTO_SIGEN: u32 = 0x00010000;
pub const MMCHS_ISE_CTO_SIGEN_SHIFT: u32 = 0x00000010;
pub const MMCHS_ISE_CTO_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_CTO_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_DCRC_SIGEN: u32 = 0x00200000;
pub const MMCHS_ISE_DCRC_SIGEN_SHIFT: u32 = 0x00000015;
pub const MMCHS_ISE_DCRC_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_DCRC_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_DEB_SIGEN: u32 = 0x00400000;
pub const MMCHS_ISE_DEB_SIGEN_SHIFT: u32 = 0x00000016;
pub const MMCHS_ISE_DEB_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_DEB_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_DMA_SIGEN: u32 = 0x00000008;
pub const MMCHS_ISE_DMA_SIGEN_SHIFT: u32 = 0x00000003;
pub const MMCHS_ISE_DMA_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_DMA_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_DTO_SIGEN: u32 = 0x00100000;
pub const MMCHS_ISE_DTO_SIGEN_SHIFT: u32 = 0x00000014;
pub const MMCHS_ISE_DTO_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_DTO_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_NULL: u32 = 0x00008000;
pub const MMCHS_ISE_NULL_SHIFT: u32 = 0x0000000F;

pub const MMCHS_ISE_OBI_SIGEN: u32 = 0x00000200;
pub const MMCHS_ISE_OBI_SIGEN_SHIFT: u32 = 0x00000009;
pub const MMCHS_ISE_OBI_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_OBI_SIGEN_MASKED: u32 = 0x0;

pub const MMCHS_ISE_TC_SIGEN: u32 = 0x00000002;
pub const MMCHS_ISE_TC_SIGEN_SHIFT: u32 = 0x00000001;
pub const MMCHS_ISE_TC_SIGEN_ENABLED: u32 = 0x1;
pub const MMCHS_ISE_TC_SIGEN_MASKED: u32 = 0x0;

// con
pub const MMCHS_CON_INIT: u32 = 0x00000002;

// stat
pub const MMCHS_STAT_CC: u32 = 0x00000001;
pub const MMCHS_STAT_CC_SHIFT: u32 = 0x00000000;

// redefs
pub const HS_MMCSD_DATALINE_RESET: u32 = MMC_SYSCTL_SRD;
pub const HS_MMCSD_CMDLINE_RESET: u32 = MMC_SYSCTL_SRC;
pub const HS_MMCSD_ALL_RESET: u32 = MMC_SYSCTL_SRA;
pub const HS_MMCSD_SUPPORT_VOLT_1P8: u32 = MMC_CAPA_VS18;
pub const HS_MMCSD_SUPPORT_VOLT_3P0: u32 = MMC_CAPA_VS30;
pub const HS_MMCSD_SUPPORT_VOLT_3P3: u32 = MMC_CAPA_VS33;
pub const HS_MMCSD_SUPPORT_DMA: u32 = MMC_CAPA_DS;
pub const HS_MMCSD_SUPPORT_HIGHSPEED: u32 = MMC_CAPA_HSS;
pub const HS_MMCSD_SUPPORT_BLOCKLEN: u32 = MMC_CAPA_MBL;

pub const HS_MMCSD_BUS_VOLT_1P8: u32 = MMCHS_HCTL_SDVS_1V8 << MMCHS_HCTL_SDVS_SHIFT;
pub const HS_MMCSD_BUS_VOLT_3P0: u32 = MMCHS_HCTL_SDVS_3V0 << MMCHS_HCTL_SDVS_SHIFT;
pub const HS_MMCSD_BUS_VOLT_3P3: u32 = MMCHS_HCTL_SDVS_3V3 << MMCHS_HCTL_SDVS_SHIFT;
pub const HS_MMCSD_BUS_POWER_ON: u32 = MMCHS_HCTL_SDBP_PWRON << MMCHS_HCTL_SDBP_SHIFT;
pub const HS_MMCSD_BUS_POWER_OFF: u32 = MMCHS_HCTL_SDBP_PWROFF << MMCHS_HCTL_SDBP_SHIFT;
pub const HS_MMCSD_BUS_HIGHSPEED: u32 = MMCHS_HCTL_HSPE_HIGHSPEED << MMCHS_HCTL_HSPE_SHIFT;
pub const HS_MMCSD_BUS_STDSPEED: u32 = MMCHS_HCTL_HSPE_NORMALSPEED << MMCHS_HCTL_HSPE_SHIFT;
pub const HS_MMCSD_INTCLOCK_ON: u32 = MMC_SYSCTL_ICE_OSCILLATE << MMC_SYSCTL_ICE_SHIFT;
pub const HS_MMCSD_INTCLOCK_OFF: u32 = MMC_SYSCTL_ICE_STOP << MMC_SYSCTL_ICE_SHIFT;

pub const HS_MMCSD_AUTOIDLE_ENABLE: u32 = MMC_SYSCONFIG_AUTOIDLE_ON << MMC_SYSCONFIG_AUTOIDLE_SHIFT;
pub const HS_MMCSD_AUTOIDLE_DISABLE: u32 =
    MMC_SYSCONFIG_AUTOIDLE_OFF << MMC_SYSCONFIG_AUTOIDLE_SHIFT;

pub const SD_CMDR_NO_RESPONSE: u32 = 0 << 16;
pub const SD_CMDR_LONG_RESPONSE: u32 = 1 << 16;
pub const SD_CMDR_SHORT_RESPONSE: u32 = 1 << 17;
pub const SD_CMDR_SHORT_RESPONSE_BUSY: u32 = 3 << 16;

use super::regs::{
    base::{CM_PER_BASE, CONTROL_MODULE_BASE},
    cm::CM_PER_MMC0_CLKCTRL,
    control::*,
};

fn controller_soft_reset() {
    unsafe {
        reg32_write_masked(
            MMC0_BASE,
            MMC_SYSCONFIG,
            MMC_SYSCONFIG_SOFTRESET,
            MMC_SYSCONFIG_SOFTRESET,
        );

        while reg32_read(MMC0_BASE, MMC_SYSSTATUS) & MMC_SYSSTATUS_RESETDONE
            != MMC_SYSSTATUS_RESETDONE
        {}
    }
}

fn lines_reset(flag: u32) {
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_SYSCTL, flag, flag);

        while reg32_read(MMC0_BASE, MMC_SYSCTL) & flag == flag {}
    }
}

fn set_supported_voltage(voltage: u32) {
    unsafe {
        reg32_write_masked(
            MMC0_BASE,
            MMC_CAPA,
            MMC_CAPA_VS18 | MMC_CAPA_VS30 | MMC_CAPA_VS33,
            voltage,
        );
    }
}

// TEMP
fn set_sd_bus_voltage() {
    unsafe {
        reg32_write_masked(
            MMC0_BASE,
            MMC_HCTL,
            MMCHS_HCTL_SDVS,       // bits 11:9
            HS_MMCSD_BUS_VOLT_3P0, // 3V
        );
    }
}

fn set_sd_bus_power(power: u32) -> Result<(), ()> {
    let mut timeout = 0xFFFFF;
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_HCTL, MMCHS_HCTL_SDBP, power);

        if (power == HS_MMCSD_BUS_POWER_ON) {
            while reg32_read_masked(MMC0_BASE, MMC_HCTL, MMCHS_HCTL_SDBP) != HS_MMCSD_BUS_POWER_ON {
                timeout -= 1;
                if timeout == 0 {
                    return Err(());
                }
            }
        }

        Ok(())
    }
}

fn is_internal_clock_stable(mut retry: u32) -> bool {
    unsafe {
        let mut reg = 0;
        while retry > 0 {
            reg = reg32_read_masked(MMC0_BASE, MMC_SYSCTL, MMC_SYSCTL_ICS) >> MMC_SYSCTL_ICS_SHIFT;
            retry -= 1;
            if (reg == 1) {
                break;
            }
        }

        return reg == 1;
    }
}

fn is_cmd_complete(mut retry: u32) -> bool {
    unsafe {
        while retry > 0 {
            let reg = reg32_read_masked(MMC0_BASE, MMC_STAT, MMCHS_STAT_CC) >> MMCHS_STAT_CC_SHIFT;
            if reg == 1 {
                // Clear command complete flag
                reg32_write_masked(MMC0_BASE, MMC_STAT, MMCHS_STAT_CC, MMCHS_STAT_CC);
                return true;
            }
            retry -= 1;
        }
        false
    }
}

fn internal_clock(power: u32) -> Result<(), ()> {
    unsafe {
        let reg = reg32_read_masked(MMC0_BASE, MMC_SYSCTL, !MMC_SYSCTL_ICE);
        reg32_write(MMC0_BASE, MMC_SYSCTL, reg | power);

        if power == HS_MMCSD_INTCLOCK_ON {
            if !is_internal_clock_stable(0xFFFFF) {
                Err(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}

fn send_init_strean() -> Result<(), ()> {
    intr_status_enable(HS_MMCSD_SIGEN_CMDCOMP);

    // initialize the init command
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_CON, MMCHS_CON_INIT, MMCHS_CON_INIT);
        reg32_write(MMC0_BASE, MMC_CMD, 0x0);

        // wait 1ms
        for _ in 0..1000000 {
            asm::nop();
        }

        // set SD_STAT[0] to 0x1
        reg32_write_masked(MMC0_BASE, MMC_STAT, 0x1, 0x1);

        // let status = is_cmd_complete(0xFFFF);
        // if !status {
        //     return Err(());
        // }
        reg32_clear_bits(MMC0_BASE, MMC_CON, MMCHS_CON_INIT);
        intr_status_clear(0xFFFFFFFF);
        Ok(())
    }
}

fn intr_status_enable(flag: u32) {
    unsafe {
        reg32_write(MMC0_BASE, MMC_IE, flag);
    }
}

fn intr_status_clear(flag: u32) {
    unsafe {
        reg32_write(MMC0_BASE, MMC_STAT, flag);
    }
}

fn system_config(config: u32) {
    unsafe {
        reg32_write_masked(
            MMC0_BASE,
            MMC_SYSCONFIG,
            (MMC_SYSCONFIG_STANDBYMODE
                | MMC_SYSCONFIG_CLOCKACTIVITY
                | MMC_SYSCONFIG_SIDLEMODE
                | MMC_SYSCONFIG_ENAWAKEUP
                | MMC_SYSCONFIG_AUTOIDLE),
            config,
        );
    }
}

fn bus_power_on() {
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_HCTL, (0b1 << 8), (0x1 << 8));
        while reg32_read_masked(MMC0_BASE, MMC_HCTL, 0b1 << 8) != (0x1 << 8) {}
    }
}

fn set_bus_width() {
    unsafe {
        reg32_clear_bits(MMC0_BASE, MMC_CON, 1 << 5);
        reg32_clear_bits(MMC0_BASE, MMC_HCTL, 0x2); // 1 bit bus width
    } //HS_MMCSD_BUS_WIDTH_1BIT
}

fn set_bus_freq(freq_in: u32, freq_out: u32, bypass: u32) {
    // enable internal clocks
    if internal_clock(HS_MMCSD_INTCLOCK_ON).is_err() {
        panic!("Failed to enable internal clock");
    }
    println!("internal clock enabled, done for now");
    if bypass == 0 {
        let mut clkd = freq_in / freq_out;
        clkd = if clkd < 2 { 2 } else { clkd };
        clkd = if clkd > 1023 { 1023 } else { clkd };

        /* Do not cross the required freq */
        while ((freq_in / clkd) > freq_out) {
            if (clkd == 1023) {
                /* Return when we cannot set the clock freq */
                panic!("Cannot set the clock freq");
            }

            clkd += 1;
        }
        unsafe {
            let reg_val = reg32_read_masked(MMC0_BASE, MMC_SYSCTL, !MMC_SYSCTL_CLKD);
            reg32_write(
                MMC0_BASE,
                MMC_SYSCTL,
                reg_val | (clkd << MMC_SYSCTL_CLKD_SHIFT),
            );

            if !is_internal_clock_stable(0xFFFFF) {
                panic!("Failed to set internal clock after setting new divider");
            }

            reg32_write_masked(MMC0_BASE, MMC_SYSCTL, MMC_SYSCTL_CEN, MMC_SYSCTL_CEN);
        }
    }
}

fn send_cmd(cmd: u32, arg: u32) -> u32 {
    let cmdr = match cmd {
        0 => SD_CMDR_NO_RESPONSE,
        5 => SD_CMDR_SHORT_RESPONSE,
        8 => SD_CMDR_SHORT_RESPONSE_BUSY,
        _ => SD_CMDR_NO_RESPONSE,
    };

    unsafe {
        // wait if command line is busy
        while reg32_read_masked(MMC0_BASE, MMC_PSTATE, MMCHS_STAT_CC) != 0x0 {}

        // make sure status is clear
        reg32_write(MMC0_BASE, MMC_STAT, 0xFFFFFFFF);
        reg32_write(MMC0_BASE, MMC_ARG, arg);
        reg32_write(MMC0_BASE, MMC_CMD, (cmd << 24) | cmdr); // CMD load, start command

        // wait for command to complete
        // while reg32_read_masked(MMC0_BASE, MMC_STAT, MMCHS_STAT_CC) != 0x1 {}
        if !is_cmd_complete(0xFFFFF) {
            panic!("Command failed to complete");
        }

        println!(
            "Command {} finished, stat reg: {:x}",
            cmd,
            reg32_read(MMC0_BASE, MMC_STAT)
        );

        return reg32_read(MMC0_BASE, MMC_RSP10);
    }
}

pub fn mux_pins() {
    unsafe {
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_DAT3,
            0 << CONTROL_CONF_MMC0_DAT3_CONF_MMC0_DAT3_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_DAT3_CONF_MMC0_DAT3_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT3_CONF_MMC0_DAT3_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT3_CONF_MMC0_DAT3_RXACTIVE_SHIFT,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_DAT2,
            0 << CONTROL_CONF_MMC0_DAT2_CONF_MMC0_DAT2_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_DAT2_CONF_MMC0_DAT2_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT2_CONF_MMC0_DAT2_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT2_CONF_MMC0_DAT2_RXACTIVE_SHIFT,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_DAT1,
            0 << CONTROL_CONF_MMC0_DAT1_CONF_MMC0_DAT1_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_DAT1_CONF_MMC0_DAT1_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT1_CONF_MMC0_DAT1_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT1_CONF_MMC0_DAT1_RXACTIVE_SHIFT,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_DAT0,
            0 << CONTROL_CONF_MMC0_DAT0_CONF_MMC0_DAT0_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_DAT0_CONF_MMC0_DAT0_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT0_CONF_MMC0_DAT0_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_DAT0_CONF_MMC0_DAT0_RXACTIVE_SHIFT,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_CLK,
            0 << CONTROL_CONF_MMC0_CLK_CONF_MMC0_CLK_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_CLK_CONF_MMC0_CLK_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_CLK_CONF_MMC0_CLK_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_CLK_CONF_MMC0_CLK_RXACTIVE_SHIFT,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_CONF_MMC0_CMD,
            0 << CONTROL_CONF_MMC0_CMD_CONF_MMC0_CMD_MMODE_SHIFT
                | 0 << CONTROL_CONF_MMC0_CMD_CONF_MMC0_CMD_PUDEN_SHIFT
                | 1 << CONTROL_CONF_MMC0_CMD_CONF_MMC0_CMD_PUTYPESEL_SHIFT
                | 1 << CONTROL_CONF_MMC0_CMD_CONF_MMC0_CMD_RXACTIVE_SHIFT,
        );

        // SPI mux?
    }
}

pub fn enable_module_clock() {
    unsafe {
        reg32_write_masked(CM_PER_BASE, CM_PER_MMC0_CLKCTRL, 0x3, 0x2);

        while reg32_read_masked(CM_PER_BASE, CM_PER_MMC0_CLKCTRL, 0x3 << 16) != 0x0 {}
    }
}

pub fn controller_init() {
    // soft reset controller
    controller_soft_reset();
    lines_reset(HS_MMCSD_ALL_RESET);
    println!("lines reset");
    set_supported_voltage(HS_MMCSD_SUPPORT_VOLT_1P8 | HS_MMCSD_SUPPORT_VOLT_3P0);
    system_config(HS_MMCSD_AUTOIDLE_ENABLE);
    set_bus_width();

    set_sd_bus_voltage();
    println!("about to power on bus");
    set_sd_bus_power(HS_MMCSD_BUS_POWER_ON).expect("Failed to power on SD bus");
    println!("bus powered on");

    // set bus frequency
    println!("setting bus freq");
    set_bus_freq(MMCSD_IN_FREQ, MMCSD_INIT_FREQ, 0);
    println!("bus freq set");
    println!("sending init stream");
    send_init_strean().expect("Failed to send init stream");
    println!("init stream sent, mmc should be ready");
}

pub fn init() {
    println!("--------------------------");
    println!("--------------------------");
    println!("--------------------------");

    println!("START MUX_PINS");
    mux_pins();
    println!("END MUX_PINS");
    println!("START MODULE_CLOCK");
    enable_module_clock();
    println!("END MODULE_CLOCK");
    println!("START CONTROLLER_INIT");
    controller_init();
    println!("END CONTROLLER_INIT");

    println!("--------------------------");
    println!("--------------------------");
    println!("--------------------------");
    println!("Sending CMD0");
    send_cmd(0, 0);
    println!("CMD0 sent");
    println!("Sending CMD5");
    // send_cmd(5, 0);
    // read status register
    // let mut reg;
    // unsafe {
    //     reg = reg32_read(MMC0_BASE, MMC_STAT);
    // }
    // println!(
    //     "CMD5 sent {}, CC: {}, CTO: {}",
    //     reg,
    //     reg & 0x1,
    //     (reg >> 16) & 0x1
    // );

    // set SD_SYSCTL[25] to 1 and wait until it resets
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_SYSCTL, 1 << 25, 1 << 25);
        while reg32_read_masked(MMC0_BASE, MMC_SYSCTL, 1 << 25) == 0 << 25 {} // wait for 1 first
        while reg32_read_masked(MMC0_BASE, MMC_SYSCTL, 1 << 25) == 1 << 25 {} // wait for 0
    }
    println!("I'm done here!");

    // println!("CMD5 sent {}", res);
    println!("Sending CMD8");
    let response = send_cmd(8, 0x1AA);
    println!("CMD8 sent");
    println!("Response: {}", response);
    if response & 0xFF != 0xAA {
        panic!("Card doesn't support 2.7-3.6V");
    }

    // let retry = 0xFFFFF;
    // while retry > 0 {
    //     send_cmd(55, 0);
    //     let response = send_cmd(41, 0x40FF8000);
    //     if response & (1 << 31) == 1 {
    //         break;
    //     }
    //     println!("Response: {}", response);
    //     retry -= 1;
    // }
}
