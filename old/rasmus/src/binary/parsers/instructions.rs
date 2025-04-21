#![allow(non_upper_case_globals)]

use nom::{
    bytes::complete::{tag, take},
    IResult as NomResult, Slice,
};

use crate::{
    binary::{
        parse_trait::ParseWithNom,
        parser_helpers::{parse, parse_all_to_vec},
    },
    entities::{
        instructions::{
            BlockInstructionType, BlockType, ExpressionType, IfElseInstructionType,
            IfInstructionType, InstructionType, LoopInstructionType,
        },
        types::{Byte, S33Type, U32Type},
    },
};

use super::types::recognize_type;

// Control Instructions
const OPCODE_UNREACHABLE: Byte = 0x00;
const OPCODE_NOP: Byte = 0x01;
const OPCODE_BLOCK: Byte = 0x02;
const OPCODE_END: Byte = 0x0B;
const OPCODE_LOOP: Byte = 0x03;
const OPCODE_IF_ELSE: Byte = 0x04;
const OPCODE_ELSE: Byte = 0x05;
const OPCODE_BR: Byte = 0x0C;
const OPCODE_BR_IF: Byte = 0x0D;
const OPCODE_BR_TABLE: Byte = 0x0E;
const OPCODE_RETURN: Byte = 0x0F;
const OPCODE_CALL: Byte = 0x10;
const OPCODE_CALL_INDIRECT: Byte = 0x11;

// Reference Instructions
const OPCODE_REF_NULL: Byte = 0xD0;
const OPCODE_REF_IS_NULL: Byte = 0xD1;
const OPCODE_REF_FUNC: Byte = 0xD2;

// Parametric Instructions
const OPCODE_DROP: Byte = 0x1A;
const OPCODE_SELECT: Byte = 0x1B;
const OPCODE_SELECT_VEC: Byte = 0x1C;

// Variable Instructions
const OPCODE_LOCAL_GET: Byte = 0x20;
const OPCODE_LOCAL_SET: Byte = 0x21;
const OPCODE_LOCAL_TEE: Byte = 0x22;
const OPCODE_GLOBAL_GET: Byte = 0x23;
const OPCODE_GLOBAL_SET: Byte = 0x24;

// Table Instructions
const OPCODE_TABLE_GET: Byte = 0x25;
const OPCODE_TABLE_SET: Byte = 0x26;
const OPCODE_OTHER: Byte = 0xFC;
const BYTECODE_TABLE_INIT: U32Type = U32Type(12);
const BYTECODE_TABLE_DROP: U32Type = U32Type(13);
const BYTECODE_TABLE_COPY: U32Type = U32Type(14);
const BYTECODE_TABLE_GROW: U32Type = U32Type(15);
const BYTECODE_TABLE_SIZE: U32Type = U32Type(16);
const BYTECODE_TABLE_FILL: U32Type = U32Type(17);

// Memory Instructions
const OPCODE_I32_LOAD: Byte = 0x28;
const OPCODE_I64_LOAD: Byte = 0x29;
const OPCODE_F32_LOAD: Byte = 0x2A;
const OPCODE_F64_LOAD: Byte = 0x2B;
const OPCODE_I32_LOAD_8_S: Byte = 0x2C;
const OPCODE_I32_LOAD_8_U: Byte = 0x2D;
const OPCODE_I32_LOAD_16_S: Byte = 0x2E;
const OPCODE_I32_LOAD_16_U: Byte = 0x2F;
const OPCODE_I64_LOAD_8_S: Byte = 0x30;
const OPCODE_I64_LOAD_8_U: Byte = 0x31;
const OPCODE_I64_LOAD_16_S: Byte = 0x32;
const OPCODE_I64_LOAD_16_U: Byte = 0x33;
const OPCODE_I64_LOAD_32_S: Byte = 0x34;
const OPCODE_I64_LOAD_32_U: Byte = 0x35;
const OPCODE_I32_STORE: Byte = 0x36;
const OPCODE_I64_STORE: Byte = 0x37;
const OPCODE_F32_STORE: Byte = 0x38;
const OPCODE_F64_STORE: Byte = 0x39;
const OPCODE_I32_STORE_8: Byte = 0x3A;
const OPCODE_I32_STORE_16: Byte = 0x3B;
const OPCODE_I64_STORE_8: Byte = 0x3C;
const OPCODE_I64_STORE_16: Byte = 0x3D;
const OPCODE_I64_STORE_32: Byte = 0x3E;
const OPCODE_MEMORY_SIZE: Byte = 0x3F;
const OPCODE_MEMORY_GROW: Byte = 0x40;
const BYTECODE_MEMORY_INIT: U32Type = U32Type(8);
const BYTECODE_DATA_DROP: U32Type = U32Type(9);
const BYTECODE_MEMORY_COPY: U32Type = U32Type(10);
const BYTECODE_MEMORY_FILL: U32Type = U32Type(11);

// Numeric Instructions
const OPCODE_I32_CONST: Byte = 0x41;
const OPCODE_I64_CONST: Byte = 0x42;
const OPCODE_F32_CONST: Byte = 0x43;
const OPCODE_F64_CONST: Byte = 0x44;
const OPCODE_I32_EQZ: Byte = 0x45;
const OPCODE_I32_EQ: Byte = 0x46;
const OPCODE_I32_NE: Byte = 0x47;
const OPCODE_I32_LT_S: Byte = 0x48;
const OPCODE_I32_LT_U: Byte = 0x49;
const OPCODE_I32_GT_S: Byte = 0x4A;
const OPCODE_I32_GT_U: Byte = 0x4B;
const OPCODE_I32_LE_S: Byte = 0x4C;
const OPCODE_I32_LE_U: Byte = 0x4D;
const OPCODE_I32_GE_S: Byte = 0x4E;
const OPCODE_I32_GE_U: Byte = 0x4F;
const OPCODE_I64_EQZ: Byte = 0x50;
const OPCODE_I64_EQ: Byte = 0x51;
const OPCODE_I64_NE: Byte = 0x52;
const OPCODE_I64_LT_S: Byte = 0x53;
const OPCODE_I64_LT_U: Byte = 0x54;
const OPCODE_I64_GT_S: Byte = 0x55;
const OPCODE_I64_GT_U: Byte = 0x56;
const OPCODE_I64_LE_S: Byte = 0x57;
const OPCODE_I64_LE_U: Byte = 0x58;
const OPCODE_I64_GE_S: Byte = 0x59;
const OPCODE_I64_GE_U: Byte = 0x5A;
const OPCODE_F32_EQ: Byte = 0x5B;
const OPCODE_F32_NE: Byte = 0x5C;
const OPCODE_F32_LT: Byte = 0x5D;
const OPCODE_F32_GT: Byte = 0x5E;
const OPCODE_F32_LE: Byte = 0x5F;
const OPCODE_F32_GE: Byte = 0x60;
const OPCODE_F64_EQ: Byte = 0x61;
const OPCODE_F64_NE: Byte = 0x62;
const OPCODE_F64_LT: Byte = 0x63;
const OPCODE_F64_GT: Byte = 0x64;
const OPCODE_F64_LE: Byte = 0x65;
const OPCODE_F64_GE: Byte = 0x66;
const OPCODE_I32_CLZ: Byte = 0x67;
const OPCODE_I32_CTZ: Byte = 0x68;
const OPCODE_I32_POPCNT: Byte = 0x69;
const OPCODE_I32_ADD: Byte = 0x6A;
const OPCODE_I32_SUB: Byte = 0x6B;
const OPCODE_I32_MUL: Byte = 0x6C;
const OPCODE_I32_DIV_S: Byte = 0x6D;
const OPCODE_I32_DIV_U: Byte = 0x6E;
const OPCODE_I32_REM_S: Byte = 0x6F;
const OPCODE_I32_REM_U: Byte = 0x70;
const OPCODE_I32_AND: Byte = 0x71;
const OPCODE_I32_OR: Byte = 0x72;
const OPCODE_I32_XOR: Byte = 0x73;
const OPCODE_I32_SHL: Byte = 0x74;
const OPCODE_I32_SHR_S: Byte = 0x75;
const OPCODE_I32_SHR_U: Byte = 0x76;
const OPCODE_I32_ROTL: Byte = 0x77;
const OPCODE_I32_ROTR: Byte = 0x78;
const OPCODE_I64_CLZ: Byte = 0x79;
const OPCODE_I64_CTZ: Byte = 0x7A;
const OPCODE_I64_POPCNT: Byte = 0x7B;
const OPCODE_I64_ADD: Byte = 0x7C;
const OPCODE_I64_SUB: Byte = 0x7D;
const OPCODE_I64_MUL: Byte = 0x7E;
const OPCODE_I64_DIV_S: Byte = 0x7F;
const OPCODE_I64_DIV_U: Byte = 0x80;
const OPCODE_I64_REM_S: Byte = 0x81;
const OPCODE_I64_REM_U: Byte = 0x82;
const OPCODE_I64_AND: Byte = 0x83;
const OPCODE_I64_OR: Byte = 0x84;
const OPCODE_I64_XOR: Byte = 0x85;
const OPCODE_I64_SHL: Byte = 0x86;
const OPCODE_I64_SHR_S: Byte = 0x87;
const OPCODE_I64_SHR_U: Byte = 0x88;
const OPCODE_I64_ROTL: Byte = 0x89;
const OPCODE_I64_ROTR: Byte = 0x8A;
const OPCODE_F32_ABS: Byte = 0x8B;
const OPCODE_F32_NEG: Byte = 0x8C;
const OPCODE_F32_CEIL: Byte = 0x8D;
const OPCODE_F32_FLOOR: Byte = 0x8E;
const OPCODE_F32_TRUNC: Byte = 0x8F;
const OPCODE_F32_NEAREST: Byte = 0x90;
const OPCODE_F32_SQRT: Byte = 0x91;
const OPCODE_F32_ADD: Byte = 0x92;
const OPCODE_F32_SUB: Byte = 0x93;
const OPCODE_F32_MUL: Byte = 0x94;
const OPCODE_F32_DIV: Byte = 0x95;
const OPCODE_F32_MIN: Byte = 0x96;
const OPCODE_F32_MAX: Byte = 0x97;
const OPCODE_F32_COPYSIGN: Byte = 0x98;
const OPCODE_F64_ABS: Byte = 0x99;
const OPCODE_F64_NEG: Byte = 0x9A;
const OPCODE_F64_CEIL: Byte = 0x9B;
const OPCODE_F64_FLOOR: Byte = 0x9C;
const OPCODE_F64_TRUNC: Byte = 0x9D;
const OPCODE_F64_NEAREST: Byte = 0x9E;
const OPCODE_F64_SQRT: Byte = 0x9F;
const OPCODE_F64_ADD: Byte = 0xA0;
const OPCODE_F64_SUB: Byte = 0xA1;
const OPCODE_F64_MUL: Byte = 0xA2;
const OPCODE_F64_DIV: Byte = 0xA3;
const OPCODE_F64_MIN: Byte = 0xA4;
const OPCODE_F64_MAX: Byte = 0xA5;
const OPCODE_F64_COPYSIGN: Byte = 0xA6;
const OPCODE_I32_WRAP_I64: Byte = 0xA7;
const OPCODE_I32_TRUNC_F32_S: Byte = 0xA8;
const OPCODE_I32_TRUNC_F32_U: Byte = 0xA9;
const OPCODE_I32_TRUNC_F64_S: Byte = 0xAA;
const OPCODE_I32_TRUNC_F64_U: Byte = 0xAB;
const OPCODE_I64_EXTEND_I32_S: Byte = 0xAC;
const OPCODE_I64_EXTEND_I32_U: Byte = 0xAD;
const OPCODE_I64_TRUNC_F32_S: Byte = 0xAE;
const OPCODE_I64_TRUNC_F32_U: Byte = 0xAF;
const OPCODE_I64_TRUNC_F64_S: Byte = 0xB0;
const OPCODE_I64_TRUNC_F64_U: Byte = 0xB1;
const OPCODE_F32_CONVERT_I32_S: Byte = 0xB2;
const OPCODE_F32_CONVERT_I32_U: Byte = 0xB3;
const OPCODE_F32_CONVERT_I64_S: Byte = 0xB4;
const OPCODE_F32_CONVERT_I64_U: Byte = 0xB5;
const OPCODE_F32_DEMOTE_F64: Byte = 0xB6;
const OPCODE_F64_CONVERT_I32_S: Byte = 0xB7;
const OPCODE_F64_CONVERT_I32_U: Byte = 0xB8;
const OPCODE_F64_CONVERT_I64_S: Byte = 0xB9;
const OPCODE_F64_CONVERT_I64_U: Byte = 0xBA;
const OPCODE_F64_PROMOTE_F32: Byte = 0xBB;
const OPCODE_I32_REINTERPRET_F32: Byte = 0xBC;
const OPCODE_I64_REINTERPRET_F64: Byte = 0xBD;
const OPCODE_F32_REINTERPRET_I32: Byte = 0xBE;
const OPCODE_F64_REINTERPRET_I64: Byte = 0xBF;
const OPCODE_I32_EXTEND_8_S: Byte = 0xC0;
const OPCODE_I32_EXTEND_16_S: Byte = 0xC1;
const OPCODE_I64_EXTEND_8_S: Byte = 0xC2;
const OPCODE_I64_EXTEND_16_S: Byte = 0xC3;
const OPCODE_I64_EXTEND_32_S: Byte = 0xC4;
const BYTE_PREFIX_I32_TRUNC_SAT_F32_S: U32Type = U32Type(0);
const BYTE_PREFIX_I32_TRUNC_SAT_F32_U: U32Type = U32Type(1);
const BYTE_PREFIX_I32_TRUNC_SAT_F64_S: U32Type = U32Type(2);
const BYTE_PREFIX_I32_TRUNC_SAT_F64_U: U32Type = U32Type(3);
const BYTE_PREFIX_I64_TRUNC_SAT_F32_S: U32Type = U32Type(4);
const BYTE_PREFIX_I64_TRUNC_SAT_F32_U: U32Type = U32Type(5);
const BYTE_PREFIX_I64_TRUNC_SAT_F64_S: U32Type = U32Type(6);
const BYTE_PREFIX_I64_TRUNC_SAT_F64_U: U32Type = U32Type(7);

// Vector Instuctions
const OPCODE_VECTOR_INSTRUCTIONS: Byte = 0xFD;
const BYTE_PREFIX_V128_LOAD: U32Type = U32Type(0);
const BYTE_PREFIX_V128_LOAD_8x8_S: U32Type = U32Type(1);
const BYTE_PREFIX_V128_LOAD_8x8_U: U32Type = U32Type(2);
const BYTE_PREFIX_V128_LOAD_16x4_S: U32Type = U32Type(3);
const BYTE_PREFIX_V128_LOAD_16x4_U: U32Type = U32Type(4);
const BYTE_PREFIX_V128_LOAD_32x2_S: U32Type = U32Type(5);
const BYTE_PREFIX_V128_LOAD_32x2_U: U32Type = U32Type(6);
const BYTE_PREFIX_V128_LOAD_8_SPLAT: U32Type = U32Type(7);
const BYTE_PREFIX_V128_LOAD_16_SPLAT: U32Type = U32Type(8);
const BYTE_PREFIX_V128_LOAD_32_SPLAT: U32Type = U32Type(9);
const BYTE_PREFIX_V128_LOAD_64_SPLAT: U32Type = U32Type(10);
const BYTE_PREFIX_V128_LOAD_32_ZERO: U32Type = U32Type(92);
const BYTE_PREFIX_V128_LOAD_64_ZERO: U32Type = U32Type(93);
const BYTE_PREFIX_V128_STORE: U32Type = U32Type(11);
const BYTE_PREFIX_V128_LOAD_8_LANE: U32Type = U32Type(84);
const BYTE_PREFIX_V128_LOAD_16_LANE: U32Type = U32Type(85);
const BYTE_PREFIX_V128_LOAD_32_LANE: U32Type = U32Type(86);
const BYTE_PREFIX_V128_LOAD_64_LANE: U32Type = U32Type(87);
const BYTE_PREFIX_V128_STORE_8_LANE: U32Type = U32Type(88);
const BYTE_PREFIX_V128_STORE_16_LANE: U32Type = U32Type(89);
const BYTE_PREFIX_V128_STORE_32_LANE: U32Type = U32Type(90);
const BYTE_PREFIX_V128_STORE_64_LANE: U32Type = U32Type(91);
const BYTE_PREFIX_V128_CONST: U32Type = U32Type(12);
const BYTE_PREFIX_I8x16_SHUFFLE: U32Type = U32Type(13);
const BYTE_PREFIX_I8x16_EXTRACT_LANE_S: U32Type = U32Type(21);
const BYTE_PREFIX_I8x16_EXTRACT_LANE_U: U32Type = U32Type(22);
const BYTE_PREFIX_I8x16_REPLACE_LANE: U32Type = U32Type(23);
const BYTE_PREFIX_I16x8_EXTRACT_LANE_S: U32Type = U32Type(24);
const BYTE_PREFIX_I16x8_EXTRACT_LANE_U: U32Type = U32Type(25);
const BYTE_PREFIX_I16x8_REPLACE_LANE: U32Type = U32Type(26);
const BYTE_PREFIX_I32x4_EXTRACT_LANE: U32Type = U32Type(27);
const BYTE_PREFIX_I32x4_REPLACE_LANE: U32Type = U32Type(28);
const BYTE_PREFIX_I64x2_EXTRACT_LANE: U32Type = U32Type(29);
const BYTE_PREFIX_I64x2_REPLACE_LANE: U32Type = U32Type(30);
const BYTE_PREFIX_F32x4_EXTRACT_LANE: U32Type = U32Type(31);
const BYTE_PREFIX_F32x4_REPLACE_LANE: U32Type = U32Type(32);
const BYTE_PREFIX_F64x2_EXTRACT_LANE: U32Type = U32Type(33);
const BYTE_PREFIX_F64x2_REPLACE_LANE: U32Type = U32Type(34);
const BYTE_PREFIX_I8x16_SWIZZLE: U32Type = U32Type(14);
const BYTE_PREFIX_I8x16_SPLAT: U32Type = U32Type(15);
const BYTE_PREFIX_I16x8_SPLAT: U32Type = U32Type(16);
const BYTE_PREFIX_I32x4_SPLAT: U32Type = U32Type(17);
const BYTE_PREFIX_I64x2_SPLAT: U32Type = U32Type(18);
const BYTE_PREFIX_F32x4_SPLAT: U32Type = U32Type(19);
const BYTE_PREFIX_F64x2_SPLAT: U32Type = U32Type(20);
const BYTE_PREFIX_I8x16_EQ: U32Type = U32Type(35);
const BYTE_PREFIX_I8x16_NE: U32Type = U32Type(36);
const BYTE_PREFIX_I8x16_LT_S: U32Type = U32Type(37);
const BYTE_PREFIX_I8x16_LT_U: U32Type = U32Type(38);
const BYTE_PREFIX_I8x16_GT_S: U32Type = U32Type(39);
const BYTE_PREFIX_I8x16_GT_U: U32Type = U32Type(40);
const BYTE_PREFIX_I8x16_LE_S: U32Type = U32Type(41);
const BYTE_PREFIX_I8x16_LE_U: U32Type = U32Type(42);
const BYTE_PREFIX_I8x16_GE_S: U32Type = U32Type(43);
const BYTE_PREFIX_I8x16_GE_U: U32Type = U32Type(44);
const BYTE_PREFIX_I16x8_EQ: U32Type = U32Type(45);
const BYTE_PREFIX_I16x8_NE: U32Type = U32Type(46);
const BYTE_PREFIX_I16x8_LT_S: U32Type = U32Type(47);
const BYTE_PREFIX_I16x8_LT_U: U32Type = U32Type(48);
const BYTE_PREFIX_I16x8_GT_S: U32Type = U32Type(49);
const BYTE_PREFIX_I16x8_GT_U: U32Type = U32Type(50);
const BYTE_PREFIX_I16x8_LE_S: U32Type = U32Type(51);
const BYTE_PREFIX_I16x8_LE_U: U32Type = U32Type(52);
const BYTE_PREFIX_I16x8_GE_S: U32Type = U32Type(53);
const BYTE_PREFIX_I16x8_GE_U: U32Type = U32Type(54);
const BYTE_PREFIX_I32x4_EQ: U32Type = U32Type(55);
const BYTE_PREFIX_I32x4_NE: U32Type = U32Type(56);
const BYTE_PREFIX_I32x4_LT_S: U32Type = U32Type(57);
const BYTE_PREFIX_I32x4_LT_U: U32Type = U32Type(58);
const BYTE_PREFIX_I32x4_GT_S: U32Type = U32Type(59);
const BYTE_PREFIX_I32x4_GT_U: U32Type = U32Type(60);
const BYTE_PREFIX_I32x4_LE_S: U32Type = U32Type(61);
const BYTE_PREFIX_I32x4_LE_U: U32Type = U32Type(62);
const BYTE_PREFIX_I32x4_GE_S: U32Type = U32Type(63);
const BYTE_PREFIX_I32x4_GE_U: U32Type = U32Type(64);
const BYTE_PREFIX_I64x2_EQ: U32Type = U32Type(214);
const BYTE_PREFIX_I64x2_NE: U32Type = U32Type(215);
const BYTE_PREFIX_I64x2_LT_S: U32Type = U32Type(216);
const BYTE_PREFIX_I64x2_GT_S: U32Type = U32Type(217);
const BYTE_PREFIX_I64x2_LE_S: U32Type = U32Type(218);
const BYTE_PREFIX_I64x2_GE_S: U32Type = U32Type(219);
const BYTE_PREFIX_F32x4_EQ: U32Type = U32Type(65);
const BYTE_PREFIX_F32x4_NE: U32Type = U32Type(66);
const BYTE_PREFIX_F32x4_LT: U32Type = U32Type(67);
const BYTE_PREFIX_F32x4_GT: U32Type = U32Type(68);
const BYTE_PREFIX_F32x4_LE: U32Type = U32Type(69);
const BYTE_PREFIX_F32x4_GE: U32Type = U32Type(70);
const BYTE_PREFIX_F64x2_EQ: U32Type = U32Type(71);
const BYTE_PREFIX_F64x2_NE: U32Type = U32Type(72);
const BYTE_PREFIX_F64x2_LT: U32Type = U32Type(73);
const BYTE_PREFIX_F64x2_GT: U32Type = U32Type(74);
const BYTE_PREFIX_F64x2_LE: U32Type = U32Type(75);
const BYTE_PREFIX_F64x2_GE: U32Type = U32Type(76);
const BYTE_PREFIX_V128_NOT: U32Type = U32Type(77);
const BYTE_PREFIX_V128_AND: U32Type = U32Type(78);
const BYTE_PREFIX_V128_ANDNOT: U32Type = U32Type(79);
const BYTE_PREFIX_V128_OR: U32Type = U32Type(80);
const BYTE_PREFIX_V128_XOR: U32Type = U32Type(81);
const BYTE_PREFIX_V128_BITSELECT: U32Type = U32Type(82);
const BYTE_PREFIX_V128_ANYTRUE: U32Type = U32Type(83);
const BYTE_PREFIX_I8x16_ABS: U32Type = U32Type(96);
const BYTE_PREFIX_I8x16_NEG: U32Type = U32Type(97);
const BYTE_PREFIX_I8x16_POPCNT: U32Type = U32Type(98);
const BYTE_PREFIX_I8x16_ALL_TRUE: U32Type = U32Type(99);
const BYTE_PREFIX_I8x16_BITMASK: U32Type = U32Type(100);
const BYTE_PREFIX_I8x16_NARROW_I16x8_S: U32Type = U32Type(101);
const BYTE_PREFIX_I8x16_NARROW_I16x8_U: U32Type = U32Type(102);
const BYTE_PREFIX_I8x16_SHL: U32Type = U32Type(107);
const BYTE_PREFIX_I8x16_SHR_S: U32Type = U32Type(108);
const BYTE_PREFIX_I8x16_SHR_U: U32Type = U32Type(109);
const BYTE_PREFIX_I8x16_ADD: U32Type = U32Type(110);
const BYTE_PREFIX_I8x16_ADD_SAT_S: U32Type = U32Type(111);
const BYTE_PREFIX_I8x16_ADD_SAT_U: U32Type = U32Type(112);
const BYTE_PREFIX_I8x16_SUB: U32Type = U32Type(113);
const BYTE_PREFIX_I8x16_SUB_SAT_S: U32Type = U32Type(114);
const BYTE_PREFIX_I8x16_SUB_SAT_U: U32Type = U32Type(115);
const BYTE_PREFIX_I8x16_MIN_S: U32Type = U32Type(118);
const BYTE_PREFIX_I8x16_MIN_U: U32Type = U32Type(119);
const BYTE_PREFIX_I8x16_MAX_S: U32Type = U32Type(120);
const BYTE_PREFIX_I8x16_MAX_U: U32Type = U32Type(121);
const BYTE_PREFIX_I8x16_AVGR_U: U32Type = U32Type(123);
const BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_S: U32Type = U32Type(124);
const BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_U: U32Type = U32Type(125);
const BYTE_PREFIX_I16x8_ABS: U32Type = U32Type(128);
const BYTE_PREFIX_I16x8_NEG: U32Type = U32Type(129);
const BYTE_PREFIX_I16x8_Q15MULR_SAT_S: U32Type = U32Type(130);
const BYTE_PREFIX_I16x8_ALL_TRUE: U32Type = U32Type(131);
const BYTE_PREFIX_I16x8_BITMASK: U32Type = U32Type(132);
const BYTE_PREFIX_I16x8_NARROW_I32x4_S: U32Type = U32Type(133);
const BYTE_PREFIX_I16x8_NARROW_I32x4_U: U32Type = U32Type(134);
const BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_S: U32Type = U32Type(135);
const BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_S: U32Type = U32Type(136);
const BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_U: U32Type = U32Type(137);
const BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_U: U32Type = U32Type(138);
const BYTE_PREFIX_I16x8_SHL: U32Type = U32Type(139);
const BYTE_PREFIX_I16x8_SHR_S: U32Type = U32Type(140);
const BYTE_PREFIX_I16x8_SHR_U: U32Type = U32Type(141);
const BYTE_PREFIX_I16x8_ADD: U32Type = U32Type(142);
const BYTE_PREFIX_I16x8_ADD_SAT_S: U32Type = U32Type(143);
const BYTE_PREFIX_I16x8_ADD_SAT_U: U32Type = U32Type(144);
const BYTE_PREFIX_I16x8_SUB: U32Type = U32Type(145);
const BYTE_PREFIX_I16x8_SUB_SAT_S: U32Type = U32Type(146);
const BYTE_PREFIX_I16x8_SUB_SAT_U: U32Type = U32Type(147);
const BYTE_PREFIX_I16x8_MUL: U32Type = U32Type(149);
const BYTE_PREFIX_I16x8_MIN_S: U32Type = U32Type(150);
const BYTE_PREFIX_I16x8_MIN_U: U32Type = U32Type(151);
const BYTE_PREFIX_I16x8_MAX_S: U32Type = U32Type(152);
const BYTE_PREFIX_I16x8_MAX_U: U32Type = U32Type(153);
const BYTE_PREFIX_I16x8_AVGR_U: U32Type = U32Type(155);
const BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_S: U32Type = U32Type(156);
const BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_S: U32Type = U32Type(157);
const BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_U: U32Type = U32Type(158);
const BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_U: U32Type = U32Type(159);
const BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_S: U32Type = U32Type(126);
const BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_U: U32Type = U32Type(127);
const BYTE_PREFIX_I32x4_ABS: U32Type = U32Type(160);
const BYTE_PREFIX_I32x4_NEG: U32Type = U32Type(161);
const BYTE_PREFIX_I32x4_ALL_TRUE: U32Type = U32Type(163);
const BYTE_PREFIX_I32x4_BITMASK: U32Type = U32Type(164);
const BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_S: U32Type = U32Type(167);
const BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_S: U32Type = U32Type(168);
const BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_U: U32Type = U32Type(169);
const BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_U: U32Type = U32Type(170);
const BYTE_PREFIX_I32x4_SHL: U32Type = U32Type(171);
const BYTE_PREFIX_I32x4_SHR_S: U32Type = U32Type(172);
const BYTE_PREFIX_I32x4_SHR_U: U32Type = U32Type(173);
const BYTE_PREFIX_I32x4_ADD: U32Type = U32Type(174);
const BYTE_PREFIX_I32x4_SUB: U32Type = U32Type(177);
const BYTE_PREFIX_I32x4_MUL: U32Type = U32Type(181);
const BYTE_PREFIX_I32x4_MIN_S: U32Type = U32Type(182);
const BYTE_PREFIX_I32x4_MIN_U: U32Type = U32Type(183);
const BYTE_PREFIX_I32x4_MAX_S: U32Type = U32Type(184);
const BYTE_PREFIX_I32x4_MAX_U: U32Type = U32Type(185);
const BYTE_PREFIX_I32x4_DOT_I16x8_S: U32Type = U32Type(186);
const BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_S: U32Type = U32Type(188);
const BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_S: U32Type = U32Type(189);
const BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_U: U32Type = U32Type(190);
const BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_U: U32Type = U32Type(191);
const BYTE_PREFIX_I64x2_ABS: U32Type = U32Type(192);
const BYTE_PREFIX_I64x2_NEG: U32Type = U32Type(193);
const BYTE_PREFIX_I64x2_ALL_TRUE: U32Type = U32Type(195);
const BYTE_PREFIX_I64x2_BITMASK: U32Type = U32Type(196);
const BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_S: U32Type = U32Type(199);
const BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_S: U32Type = U32Type(200);
const BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_U: U32Type = U32Type(201);
const BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_U: U32Type = U32Type(202);
const BYTE_PREFIX_I64x2_SHL: U32Type = U32Type(203);
const BYTE_PREFIX_I64x2_SHR_S: U32Type = U32Type(204);
const BYTE_PREFIX_I64x2_SHR_U: U32Type = U32Type(205);
const BYTE_PREFIX_I64x2_ADD: U32Type = U32Type(206);
const BYTE_PREFIX_I64x2_SUB: U32Type = U32Type(209);
const BYTE_PREFIX_I64x2_MUL: U32Type = U32Type(213);
const BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_S: U32Type = U32Type(220);
const BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_S: U32Type = U32Type(221);
const BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_U: U32Type = U32Type(222);
const BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_U: U32Type = U32Type(223);
const BYTE_PREFIX_F32x4_CEIL: U32Type = U32Type(103);
const BYTE_PREFIX_F32x4_FLOOR: U32Type = U32Type(104);
const BYTE_PREFIX_F32x4_TRUNC: U32Type = U32Type(105);
const BYTE_PREFIX_F32x4_NEAREST: U32Type = U32Type(106);
const BYTE_PREFIX_F32x4_ABS: U32Type = U32Type(224);
const BYTE_PREFIX_F32x4_NEG: U32Type = U32Type(225);
const BYTE_PREFIX_F32x4_SQRT: U32Type = U32Type(227);
const BYTE_PREFIX_F32x4_ADD: U32Type = U32Type(228);
const BYTE_PREFIX_F32x4_SUB: U32Type = U32Type(229);
const BYTE_PREFIX_F32x4_MUL: U32Type = U32Type(230);
const BYTE_PREFIX_F32x4_DIV: U32Type = U32Type(231);
const BYTE_PREFIX_F32x4_MIN: U32Type = U32Type(232);
const BYTE_PREFIX_F32x4_MAX: U32Type = U32Type(233);
const BYTE_PREFIX_F32x4_PMIN: U32Type = U32Type(234);
const BYTE_PREFIX_F32x4_PMAX: U32Type = U32Type(235);
const BYTE_PREFIX_F64x2_CEIL: U32Type = U32Type(116);
const BYTE_PREFIX_F64x2_FLOOR: U32Type = U32Type(117);
const BYTE_PREFIX_F64x2_TRUNC: U32Type = U32Type(122);
const BYTE_PREFIX_F64x2_NEAREST: U32Type = U32Type(148);
const BYTE_PREFIX_F64x2_ABS: U32Type = U32Type(236);
const BYTE_PREFIX_F64x2_NEG: U32Type = U32Type(237);
const BYTE_PREFIX_F64x2_SQRT: U32Type = U32Type(239);
const BYTE_PREFIX_F64x2_ADD: U32Type = U32Type(240);
const BYTE_PREFIX_F64x2_SUB: U32Type = U32Type(241);
const BYTE_PREFIX_F64x2_MUL: U32Type = U32Type(242);
const BYTE_PREFIX_F64x2_DIV: U32Type = U32Type(243);
const BYTE_PREFIX_F64x2_MIN: U32Type = U32Type(244);
const BYTE_PREFIX_F64x2_MAX: U32Type = U32Type(245);
const BYTE_PREFIX_F64x2_PMIN: U32Type = U32Type(246);
const BYTE_PREFIX_F64x2_PMAX: U32Type = U32Type(247);
const BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_S: U32Type = U32Type(248);
const BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_U: U32Type = U32Type(249);
const BYTE_PREFIX_F32x4_CONVERT_I32x4_S: U32Type = U32Type(250);
const BYTE_PREFIX_F32x4_CONVERT_I32x4_U: U32Type = U32Type(251);
const BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_S_ZERO: U32Type = U32Type(252);
const BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_U_ZERO: U32Type = U32Type(253);
const BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_S: U32Type = U32Type(254);
const BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_U: U32Type = U32Type(255);
const BYTE_PREFIX_F32x4_DEMOTE_F64x2_ZERO: U32Type = U32Type(94);
const BYTE_PREFIX_F64x2_PROMOTE_LOW_F32x4: U32Type = U32Type(95);

const OP_CODE_END: Byte = 0x0B;
const OPCODE_EMPTY: Byte = 0x40;

impl ParseWithNom for IfElseInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;

        if bytes.iter().find(|b| **b == OPCODE_ELSE).is_some() {
            // if ... else ... end instruction
            let (bytes, if_instructions) = parse_all_to_vec(bytes, OPCODE_ELSE)?;
            let (bytes, else_instructions) = parse_all_to_vec(bytes, OPCODE_END)?;

            Ok((
                bytes,
                IfElseInstructionType {
                    blocktype,
                    if_instructions,
                    else_instructions,
                },
            ))
        } else {
            // if ... end (no else) instruction
            let (bytes, if_instructions) = parse_all_to_vec(bytes, OPCODE_END)?;

            Ok((
                bytes,
                IfElseInstructionType {
                    blocktype,
                    if_instructions,
                    else_instructions: vec![],
                },
            ))
        }
    }
}

impl ParseWithNom for ExpressionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, instructions) = parse_all_to_vec(bytes, OP_CODE_END)?;

        Ok((
            bytes,
            ExpressionType {
                instructions: instructions,
            },
        ))
    }
}

impl ParseWithNom for BlockType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        match bytes.get(0) {
            Some(first_byte) => {
                if *first_byte == OPCODE_EMPTY {
                    return Ok((bytes.slice(1..), Self::Empty));
                }

                if let Some(val_type) = recognize_type(*first_byte) {
                    return Ok((bytes.slice(1..), Self::ValType(val_type)));
                }

                return S33Type::parse(bytes).map(|(b, v)| (b, Self::TypeIndex(v)));
            }
            None => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

impl ParseWithNom for IfInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, if_instructions) = parse_all_to_vec(bytes, OPCODE_ELSE)?;

        Ok((
            bytes,
            IfInstructionType {
                blocktype,
                if_instructions,
            },
        ))
    }
}

impl ParseWithNom for LoopInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, instructions) = parse_all_to_vec(bytes, OPCODE_END)?;

        Ok((
            bytes,
            LoopInstructionType {
                blocktype,
                instructions,
            },
        ))
    }
}

impl ParseWithNom for BlockInstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, blocktype) = BlockType::parse(bytes)?;
        let (bytes, instructions) = parse_all_to_vec(bytes, OPCODE_END)?;

        Ok((
            bytes,
            BlockInstructionType {
                blocktype,
                instructions,
            },
        ))
    }
}

impl ParseWithNom for InstructionType {
    fn parse(bytes: &[Byte]) -> NomResult<&[Byte], Self> {
        let (bytes, opcode) = take(1usize)(bytes)?;

        match opcode[0] {
            OPCODE_UNREACHABLE => Ok((bytes, Self::Unreachable)),
            OPCODE_NOP => Ok((bytes, Self::Nop)),
            OPCODE_BLOCK => parse(bytes).map(|(b, v)| (b, Self::Block(v))),
            OPCODE_LOOP => parse(bytes).map(|(b, v)| (b, Self::Loop(v))),
            OPCODE_IF_ELSE => parse(bytes).map(|(b, v)| (b, Self::IfElse(v))),
            OPCODE_BR => parse(bytes).map(|(b, v)| (b, Self::Br(v))),
            OPCODE_BR_IF => parse(bytes).map(|(b, v)| (b, Self::BrIf(v))),
            OPCODE_BR_TABLE => parse(bytes).map(|(b, v)| (b, Self::BrTable(v))),
            OPCODE_RETURN => Ok((bytes, Self::Return)),
            OPCODE_CALL => parse(bytes).map(|(b, v)| (b, Self::Call(v))),
            OPCODE_CALL_INDIRECT => parse(bytes).map(|(b, v)| (b, Self::CallIndirect(v))),

            OPCODE_REF_NULL => parse(bytes).map(|(b, v)| (b, Self::RefNull(v))),
            OPCODE_REF_IS_NULL => Ok((bytes, Self::RefIsNull)),
            OPCODE_REF_FUNC => parse(bytes).map(|(b, v)| (b, Self::RefFunc(v))),
            OPCODE_DROP => Ok((bytes, Self::Drop)),
            OPCODE_SELECT => Ok((bytes, Self::Select)),

            OPCODE_SELECT_VEC => parse(bytes).map(|(b, v)| (b, Self::SelectVec(v))),
            OPCODE_LOCAL_GET => parse(bytes).map(|(b, v)| (b, Self::LocalGet(v))),
            OPCODE_LOCAL_SET => parse(bytes).map(|(b, v)| (b, Self::LocalSet(v))),
            OPCODE_LOCAL_TEE => parse(bytes).map(|(b, v)| (b, Self::LocalTee(v))),
            OPCODE_GLOBAL_GET => parse(bytes).map(|(b, v)| (b, Self::GlobalGet(v))),
            OPCODE_GLOBAL_SET => parse(bytes).map(|(b, v)| (b, Self::GlobalSet(v))),

            OPCODE_TABLE_GET => parse(bytes).map(|(b, v)| (b, Self::TableGet(v))),
            OPCODE_TABLE_SET => parse(bytes).map(|(b, v)| (b, Self::TableSet(v))),
            OPCODE_OTHER => parse_other(bytes),

            OPCODE_I32_LOAD => parse(bytes).map(|(b, v)| (b, Self::I32Load(v))),
            OPCODE_I64_LOAD => parse(bytes).map(|(b, v)| (b, Self::I64Load(v))),
            OPCODE_F32_LOAD => parse(bytes).map(|(b, v)| (b, Self::F32Load(v))),
            OPCODE_F64_LOAD => parse(bytes).map(|(b, v)| (b, Self::F64Load(v))),
            OPCODE_I32_LOAD_8_S => parse(bytes).map(|(b, v)| (b, Self::I32Load8S(v))),
            OPCODE_I32_LOAD_8_U => parse(bytes).map(|(b, v)| (b, Self::I32Load8U(v))),
            OPCODE_I32_LOAD_16_S => parse(bytes).map(|(b, v)| (b, Self::I32Load16S(v))),
            OPCODE_I32_LOAD_16_U => parse(bytes).map(|(b, v)| (b, Self::I32Load16U(v))),
            OPCODE_I64_LOAD_8_S => parse(bytes).map(|(b, v)| (b, Self::I64Load8S(v))),
            OPCODE_I64_LOAD_8_U => parse(bytes).map(|(b, v)| (b, Self::I64Load8U(v))),
            OPCODE_I64_LOAD_16_S => parse(bytes).map(|(b, v)| (b, Self::I64Load16S(v))),
            OPCODE_I64_LOAD_16_U => parse(bytes).map(|(b, v)| (b, Self::I64Load16U(v))),
            OPCODE_I64_LOAD_32_S => parse(bytes).map(|(b, v)| (b, Self::I64Load32S(v))),
            OPCODE_I64_LOAD_32_U => parse(bytes).map(|(b, v)| (b, Self::I64Load32U(v))),
            OPCODE_I32_STORE => parse(bytes).map(|(b, v)| (b, Self::I32Store(v))),
            OPCODE_I64_STORE => parse(bytes).map(|(b, v)| (b, Self::I64Store(v))),
            OPCODE_F32_STORE => parse(bytes).map(|(b, v)| (b, Self::F32Store(v))),
            OPCODE_F64_STORE => parse(bytes).map(|(b, v)| (b, Self::F64Store(v))),
            OPCODE_I32_STORE_8 => parse(bytes).map(|(b, v)| (b, Self::I32Store8(v))),
            OPCODE_I32_STORE_16 => parse(bytes).map(|(b, v)| (b, Self::I32Store16(v))),
            OPCODE_I64_STORE_8 => parse(bytes).map(|(b, v)| (b, Self::I64Store8(v))),
            OPCODE_I64_STORE_16 => parse(bytes).map(|(b, v)| (b, Self::I64Store16(v))),
            OPCODE_I64_STORE_32 => parse(bytes).map(|(b, v)| (b, Self::I64Store32(v))),
            OPCODE_MEMORY_SIZE => Ok((tag(&[0x00])(bytes).map(|v| v.0)?, Self::MemorySize)),
            OPCODE_MEMORY_GROW => Ok((tag(&[0x00])(bytes).map(|v| v.0)?, Self::MemoryGrow)),

            OPCODE_I32_CONST => parse(bytes).map(|(b, v)| (b, Self::I32Const(v))),
            OPCODE_I64_CONST => parse(bytes).map(|(b, v)| (b, Self::I64Const(v))),
            OPCODE_F32_CONST => parse(bytes).map(|(b, v)| (b, Self::F32Const(v))),
            OPCODE_F64_CONST => parse(bytes).map(|(b, v)| (b, Self::F64Const(v))),
            OPCODE_I32_EQZ => Ok((bytes, Self::I32Eqz)),
            OPCODE_I32_EQ => Ok((bytes, Self::I32Eq)),
            OPCODE_I32_NE => Ok((bytes, Self::I32Ne)),
            OPCODE_I32_LT_S => Ok((bytes, Self::I32LtS)),
            OPCODE_I32_LT_U => Ok((bytes, Self::I32LtU)),
            OPCODE_I32_GT_S => Ok((bytes, Self::I32GtS)),
            OPCODE_I32_GT_U => Ok((bytes, Self::I32GtU)),
            OPCODE_I32_LE_S => Ok((bytes, Self::I32LeS)),
            OPCODE_I32_LE_U => Ok((bytes, Self::I32LeU)),
            OPCODE_I32_GE_S => Ok((bytes, Self::I32GeS)),
            OPCODE_I32_GE_U => Ok((bytes, Self::I32GeU)),
            OPCODE_I64_EQZ => Ok((bytes, Self::I64Eqz)),
            OPCODE_I64_EQ => Ok((bytes, Self::I64Eq)),
            OPCODE_I64_NE => Ok((bytes, Self::I64Ne)),
            OPCODE_I64_LT_S => Ok((bytes, Self::I64LtS)),
            OPCODE_I64_LT_U => Ok((bytes, Self::I64LtU)),
            OPCODE_I64_GT_S => Ok((bytes, Self::I64GtS)),
            OPCODE_I64_GT_U => Ok((bytes, Self::I64GtU)),
            OPCODE_I64_LE_S => Ok((bytes, Self::I64LeS)),
            OPCODE_I64_LE_U => Ok((bytes, Self::I64LeU)),
            OPCODE_I64_GE_S => Ok((bytes, Self::I64GeS)),
            OPCODE_I64_GE_U => Ok((bytes, Self::I64GeU)),
            OPCODE_F32_EQ => Ok((bytes, Self::F32Eq)),
            OPCODE_F32_NE => Ok((bytes, Self::F32Ne)),
            OPCODE_F32_LT => Ok((bytes, Self::F32Lt)),
            OPCODE_F32_GT => Ok((bytes, Self::F32Gt)),
            OPCODE_F32_LE => Ok((bytes, Self::F32Le)),
            OPCODE_F32_GE => Ok((bytes, Self::F32Ge)),
            OPCODE_F64_EQ => Ok((bytes, Self::F64Eq)),
            OPCODE_F64_NE => Ok((bytes, Self::F64Ne)),
            OPCODE_F64_LT => Ok((bytes, Self::F64Lt)),
            OPCODE_F64_GT => Ok((bytes, Self::F64Gt)),
            OPCODE_F64_LE => Ok((bytes, Self::F64Le)),
            OPCODE_F64_GE => Ok((bytes, Self::F64Ge)),
            OPCODE_I32_CLZ => Ok((bytes, Self::I32Clz)),
            OPCODE_I32_CTZ => Ok((bytes, Self::I32Ctz)),
            OPCODE_I32_POPCNT => Ok((bytes, Self::I32Popcnt)),
            OPCODE_I32_ADD => Ok((bytes, Self::I32Add)),
            OPCODE_I32_SUB => Ok((bytes, Self::I32Sub)),
            OPCODE_I32_MUL => Ok((bytes, Self::I32Mul)),
            OPCODE_I32_DIV_S => Ok((bytes, Self::I32DivS)),
            OPCODE_I32_DIV_U => Ok((bytes, Self::I32DivU)),
            OPCODE_I32_REM_S => Ok((bytes, Self::I32RemS)),
            OPCODE_I32_REM_U => Ok((bytes, Self::I32RemU)),
            OPCODE_I32_AND => Ok((bytes, Self::I32And)),
            OPCODE_I32_OR => Ok((bytes, Self::I32Or)),
            OPCODE_I32_XOR => Ok((bytes, Self::I32Xor)),
            OPCODE_I32_SHL => Ok((bytes, Self::I32Shl)),
            OPCODE_I32_SHR_S => Ok((bytes, Self::I32ShrS)),
            OPCODE_I32_SHR_U => Ok((bytes, Self::I32ShrU)),
            OPCODE_I32_ROTL => Ok((bytes, Self::I32Rotl)),
            OPCODE_I32_ROTR => Ok((bytes, Self::I32Rotr)),
            OPCODE_I64_CLZ => Ok((bytes, Self::I64Clz)),
            OPCODE_I64_CTZ => Ok((bytes, Self::I64Ctz)),
            OPCODE_I64_POPCNT => Ok((bytes, Self::I64Popcnt)),
            OPCODE_I64_ADD => Ok((bytes, Self::I64Add)),
            OPCODE_I64_SUB => Ok((bytes, Self::I64Sub)),
            OPCODE_I64_MUL => Ok((bytes, Self::I64Mul)),
            OPCODE_I64_DIV_S => Ok((bytes, Self::I64DivS)),
            OPCODE_I64_DIV_U => Ok((bytes, Self::I64DivU)),
            OPCODE_I64_REM_S => Ok((bytes, Self::I64RemS)),
            OPCODE_I64_REM_U => Ok((bytes, Self::I64RemU)),
            OPCODE_I64_AND => Ok((bytes, Self::I64And)),
            OPCODE_I64_OR => Ok((bytes, Self::I64Or)),
            OPCODE_I64_XOR => Ok((bytes, Self::I64Xor)),
            OPCODE_I64_SHL => Ok((bytes, Self::I64Shl)),
            OPCODE_I64_SHR_S => Ok((bytes, Self::I64ShrS)),
            OPCODE_I64_SHR_U => Ok((bytes, Self::I64ShrU)),
            OPCODE_I64_ROTL => Ok((bytes, Self::I64Rotl)),
            OPCODE_I64_ROTR => Ok((bytes, Self::I64Rotr)),

            OPCODE_F32_ABS => Ok((bytes, Self::F32Abs)),
            OPCODE_F32_NEG => Ok((bytes, Self::F32Neg)),
            OPCODE_F32_CEIL => Ok((bytes, Self::F32Ceil)),
            OPCODE_F32_FLOOR => Ok((bytes, Self::F32Floor)),
            OPCODE_F32_TRUNC => Ok((bytes, Self::F32Trunc)),
            OPCODE_F32_NEAREST => Ok((bytes, Self::F32Nearest)),
            OPCODE_F32_SQRT => Ok((bytes, Self::F32Sqrt)),
            OPCODE_F32_ADD => Ok((bytes, Self::F32Add)),
            OPCODE_F32_SUB => Ok((bytes, Self::F32Sub)),
            OPCODE_F32_MUL => Ok((bytes, Self::F32Mul)),
            OPCODE_F32_DIV => Ok((bytes, Self::F32Div)),
            OPCODE_F32_MIN => Ok((bytes, Self::F32Min)),
            OPCODE_F32_MAX => Ok((bytes, Self::F32Max)),
            OPCODE_F32_COPYSIGN => Ok((bytes, Self::F32Copysign)),
            OPCODE_F64_ABS => Ok((bytes, Self::F64Abs)),
            OPCODE_F64_NEG => Ok((bytes, Self::F64Neg)),
            OPCODE_F64_CEIL => Ok((bytes, Self::F64Ceil)),
            OPCODE_F64_FLOOR => Ok((bytes, Self::F64Floor)),
            OPCODE_F64_TRUNC => Ok((bytes, Self::F64Trunc)),
            OPCODE_F64_NEAREST => Ok((bytes, Self::F64Nearest)),
            OPCODE_F64_SQRT => Ok((bytes, Self::F64Sqrt)),
            OPCODE_F64_ADD => Ok((bytes, Self::F64Add)),
            OPCODE_F64_SUB => Ok((bytes, Self::F64Sub)),
            OPCODE_F64_MUL => Ok((bytes, Self::F64Mul)),
            OPCODE_F64_DIV => Ok((bytes, Self::F64Div)),
            OPCODE_F64_MIN => Ok((bytes, Self::F64Min)),
            OPCODE_F64_MAX => Ok((bytes, Self::F64Max)),
            OPCODE_F64_COPYSIGN => Ok((bytes, Self::F64Copysign)),
            OPCODE_I32_WRAP_I64 => Ok((bytes, Self::I32WrapI64)),
            OPCODE_I32_TRUNC_F32_S => Ok((bytes, Self::I32TruncF32S)),
            OPCODE_I32_TRUNC_F32_U => Ok((bytes, Self::I32TruncF32U)),
            OPCODE_I32_TRUNC_F64_S => Ok((bytes, Self::I32TruncF64S)),
            OPCODE_I32_TRUNC_F64_U => Ok((bytes, Self::I32TruncF64U)),
            OPCODE_I64_EXTEND_I32_S => Ok((bytes, Self::I64ExtendI32S)),
            OPCODE_I64_EXTEND_I32_U => Ok((bytes, Self::I64ExtendI32U)),
            OPCODE_I64_TRUNC_F32_S => Ok((bytes, Self::I64TruncF32S)),
            OPCODE_I64_TRUNC_F32_U => Ok((bytes, Self::I64TruncF32U)),
            OPCODE_I64_TRUNC_F64_S => Ok((bytes, Self::I64TruncF64S)),
            OPCODE_I64_TRUNC_F64_U => Ok((bytes, Self::I64TruncF64U)),
            OPCODE_F32_CONVERT_I32_S => Ok((bytes, Self::F32ConvertI32S)),
            OPCODE_F32_CONVERT_I32_U => Ok((bytes, Self::F32ConvertI32U)),
            OPCODE_F32_CONVERT_I64_S => Ok((bytes, Self::F32ConvertI64S)),
            OPCODE_F32_CONVERT_I64_U => Ok((bytes, Self::F32ConvertI64U)),
            OPCODE_F32_DEMOTE_F64 => Ok((bytes, Self::F32DemoteF64)),
            OPCODE_F64_CONVERT_I32_S => Ok((bytes, Self::F64ConvertI32S)),
            OPCODE_F64_CONVERT_I32_U => Ok((bytes, Self::F64ConvertI32U)),
            OPCODE_F64_CONVERT_I64_S => Ok((bytes, Self::F64ConvertI64S)),
            OPCODE_F64_CONVERT_I64_U => Ok((bytes, Self::F64ConvertI64U)),
            OPCODE_F64_PROMOTE_F32 => Ok((bytes, Self::F64PromoteF32)),
            OPCODE_I32_REINTERPRET_F32 => Ok((bytes, Self::I32ReinterpretF32)),
            OPCODE_I64_REINTERPRET_F64 => Ok((bytes, Self::I64ReinterpretF64)),
            OPCODE_F32_REINTERPRET_I32 => Ok((bytes, Self::F32ReinterpretI32)),
            OPCODE_F64_REINTERPRET_I64 => Ok((bytes, Self::F64ReinterpretI64)),
            OPCODE_I32_EXTEND_8_S => Ok((bytes, Self::I32Extend8S)),
            OPCODE_I32_EXTEND_16_S => Ok((bytes, Self::I32Extend16S)),
            OPCODE_I64_EXTEND_8_S => Ok((bytes, Self::I64Extend8S)),
            OPCODE_I64_EXTEND_16_S => Ok((bytes, Self::I64Extend16S)),
            OPCODE_I64_EXTEND_32_S => Ok((bytes, Self::I64Extend32S)),

            OPCODE_VECTOR_INSTRUCTIONS => parse_vector_instruction(bytes),

            _ => Err(nom::Err::Failure(nom::error::Error::new(
                bytes,
                nom::error::ErrorKind::Fail,
            ))),
        }
    }
}

fn parse_other(bytes: &[Byte]) -> NomResult<&[Byte], InstructionType> {
    let (bytes, bytecode) = U32Type::parse(bytes)?;

    match bytecode {
        BYTE_PREFIX_I32_TRUNC_SAT_F32_S => Ok((bytes, InstructionType::I32TruncSatF32S)),
        BYTE_PREFIX_I32_TRUNC_SAT_F32_U => Ok((bytes, InstructionType::I32TruncSatF32U)),
        BYTE_PREFIX_I32_TRUNC_SAT_F64_S => Ok((bytes, InstructionType::I32TruncSatF64S)),
        BYTE_PREFIX_I32_TRUNC_SAT_F64_U => Ok((bytes, InstructionType::I32TruncSatF64U)),
        BYTE_PREFIX_I64_TRUNC_SAT_F32_S => Ok((bytes, InstructionType::I64TruncSatF32S)),
        BYTE_PREFIX_I64_TRUNC_SAT_F32_U => Ok((bytes, InstructionType::I64TruncSatF32U)),
        BYTE_PREFIX_I64_TRUNC_SAT_F64_S => Ok((bytes, InstructionType::I64TruncSatF64S)),
        BYTE_PREFIX_I64_TRUNC_SAT_F64_U => Ok((bytes, InstructionType::I64TruncSatF64U)),

        BYTECODE_MEMORY_INIT => parse(bytes).map(|(b, v)| (b, InstructionType::MemoryInit(v))),
        BYTECODE_DATA_DROP => parse(bytes).map(|(b, v)| (b, InstructionType::DataDrop(v))),
        BYTECODE_MEMORY_COPY => Ok((
            tag(&[0x00, 0x00])(bytes).map(|r| r.0)?,
            InstructionType::MemoryCopy,
        )),
        BYTECODE_MEMORY_FILL => Ok((
            tag(&[0x00])(bytes).map(|r| r.0)?,
            InstructionType::MemoryFill,
        )),
        BYTECODE_TABLE_INIT => parse(bytes).map(|(b, v)| (b, InstructionType::TableInit(v))),
        BYTECODE_TABLE_DROP => parse(bytes).map(|(b, v)| (b, InstructionType::ElemDrop(v))),
        BYTECODE_TABLE_COPY => parse(bytes).map(|(b, v)| (b, InstructionType::TableCopy(v))),
        BYTECODE_TABLE_GROW => parse(bytes).map(|(b, v)| (b, InstructionType::TableGrow(v))),
        BYTECODE_TABLE_SIZE => parse(bytes).map(|(b, v)| (b, InstructionType::TableSize(v))),
        BYTECODE_TABLE_FILL => parse(bytes).map(|(b, v)| (b, InstructionType::TableFill(v))),
        _ => Err(nom::Err::Failure(nom::error::Error::new(
            bytes,
            nom::error::ErrorKind::Fail,
        ))),
    }
}

fn parse_vector_instruction(bytes: &[Byte]) -> NomResult<&[Byte], InstructionType> {
    let (bytes, byteprefix) = U32Type::parse(bytes)?;

    match byteprefix {
        BYTE_PREFIX_V128_LOAD => parse(bytes).map(|(b, v)| (b, InstructionType::V128Load(v))),
        BYTE_PREFIX_V128_LOAD_8x8_S => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load8x8S(v)))
        }
        BYTE_PREFIX_V128_LOAD_8x8_U => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load8x8U(v)))
        }
        BYTE_PREFIX_V128_LOAD_16x4_S => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load16x4S(v)))
        }
        BYTE_PREFIX_V128_LOAD_16x4_U => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load16x4U(v)))
        }
        BYTE_PREFIX_V128_LOAD_32x2_S => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load32x2S(v)))
        }
        BYTE_PREFIX_V128_LOAD_32x2_U => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load32x2U(v)))
        }
        BYTE_PREFIX_V128_LOAD_8_SPLAT => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load8Splat(v)))
        }
        BYTE_PREFIX_V128_LOAD_16_SPLAT => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load16Splat(v)))
        }
        BYTE_PREFIX_V128_LOAD_32_SPLAT => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load32Splat(v)))
        }
        BYTE_PREFIX_V128_LOAD_64_SPLAT => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load64Splat(v)))
        }
        BYTE_PREFIX_V128_LOAD_32_ZERO => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load32Zero(v)))
        }
        BYTE_PREFIX_V128_LOAD_64_ZERO => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load64Zero(v)))
        }
        BYTE_PREFIX_V128_STORE => parse(bytes).map(|(b, v)| (b, InstructionType::V128Store(v))),
        BYTE_PREFIX_V128_LOAD_8_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load8Lane(v)))
        }
        BYTE_PREFIX_V128_LOAD_16_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load16Lane(v)))
        }
        BYTE_PREFIX_V128_LOAD_32_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load32Lane(v)))
        }
        BYTE_PREFIX_V128_LOAD_64_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Load64Lane(v)))
        }
        BYTE_PREFIX_V128_STORE_8_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Store8Lane(v)))
        }
        BYTE_PREFIX_V128_STORE_16_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Store16Lane(v)))
        }
        BYTE_PREFIX_V128_STORE_32_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Store32Lane(v)))
        }
        BYTE_PREFIX_V128_STORE_64_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::V128Store64Lane(v)))
        }
        BYTE_PREFIX_V128_CONST => parse(bytes).map(|(b, v)| (b, InstructionType::V128Const(v))),
        BYTE_PREFIX_I8x16_SHUFFLE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I8x16Shuffle(v)))
        }
        BYTE_PREFIX_I8x16_EXTRACT_LANE_S => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I8x16ExtractLaneS(v)))
        }
        BYTE_PREFIX_I8x16_EXTRACT_LANE_U => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I8x16ExtractLaneU(v)))
        }
        BYTE_PREFIX_I8x16_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I8x16ReplaceLane(v)))
        }
        BYTE_PREFIX_I16x8_EXTRACT_LANE_S => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I16x8ExtractLaneS(v)))
        }
        BYTE_PREFIX_I16x8_EXTRACT_LANE_U => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I16x8ExtractLaneU(v)))
        }
        BYTE_PREFIX_I16x8_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I16x8ReplaceLane(v)))
        }
        BYTE_PREFIX_I32x4_EXTRACT_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I32x4ExtractLane(v)))
        }
        BYTE_PREFIX_I32x4_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I32x4ReplaceLane(v)))
        }
        BYTE_PREFIX_I64x2_EXTRACT_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I64x2ExtractLane(v)))
        }
        BYTE_PREFIX_I64x2_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::I64x2ReplaceLane(v)))
        }
        BYTE_PREFIX_F32x4_EXTRACT_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::F32x4ExtractLane(v)))
        }
        BYTE_PREFIX_F32x4_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::F32x4ReplaceLane(v)))
        }
        BYTE_PREFIX_F64x2_EXTRACT_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::F64x2ExtractLane(v)))
        }
        BYTE_PREFIX_F64x2_REPLACE_LANE => {
            parse(bytes).map(|(b, v)| (b, InstructionType::F64x2ReplaceLane(v)))
        }
        BYTE_PREFIX_I8x16_SWIZZLE => Ok((bytes, InstructionType::I8x16Swizzle)),
        BYTE_PREFIX_I8x16_SPLAT => Ok((bytes, InstructionType::I8x16Splat)),
        BYTE_PREFIX_I16x8_SPLAT => Ok((bytes, InstructionType::I16x8Splat)),
        BYTE_PREFIX_I32x4_SPLAT => Ok((bytes, InstructionType::I32x4Splat)),
        BYTE_PREFIX_I64x2_SPLAT => Ok((bytes, InstructionType::I64x2Splat)),
        BYTE_PREFIX_F32x4_SPLAT => Ok((bytes, InstructionType::F32x4Splat)),
        BYTE_PREFIX_F64x2_SPLAT => Ok((bytes, InstructionType::F64x2Splat)),
        BYTE_PREFIX_I8x16_EQ => Ok((bytes, InstructionType::I8x16Eq)),
        BYTE_PREFIX_I8x16_NE => Ok((bytes, InstructionType::I8x16Ne)),
        BYTE_PREFIX_I8x16_LT_S => Ok((bytes, InstructionType::I8x16LtS)),
        BYTE_PREFIX_I8x16_LT_U => Ok((bytes, InstructionType::I8x16LtU)),
        BYTE_PREFIX_I8x16_GT_S => Ok((bytes, InstructionType::I8x16GtS)),
        BYTE_PREFIX_I8x16_GT_U => Ok((bytes, InstructionType::I8x16GtU)),
        BYTE_PREFIX_I8x16_LE_S => Ok((bytes, InstructionType::I8x16LeS)),
        BYTE_PREFIX_I8x16_LE_U => Ok((bytes, InstructionType::I8x16LeU)),
        BYTE_PREFIX_I8x16_GE_S => Ok((bytes, InstructionType::I8x16GeS)),
        BYTE_PREFIX_I8x16_GE_U => Ok((bytes, InstructionType::I8x16GeU)),
        BYTE_PREFIX_I16x8_EQ => Ok((bytes, InstructionType::I16x8Eq)),
        BYTE_PREFIX_I16x8_NE => Ok((bytes, InstructionType::I16x8Ne)),
        BYTE_PREFIX_I16x8_LT_S => Ok((bytes, InstructionType::I16x8LtS)),
        BYTE_PREFIX_I16x8_LT_U => Ok((bytes, InstructionType::I16x8LtU)),
        BYTE_PREFIX_I16x8_GT_S => Ok((bytes, InstructionType::I16x8GtS)),
        BYTE_PREFIX_I16x8_GT_U => Ok((bytes, InstructionType::I16x8GtU)),
        BYTE_PREFIX_I16x8_LE_S => Ok((bytes, InstructionType::I16x8LeS)),
        BYTE_PREFIX_I16x8_LE_U => Ok((bytes, InstructionType::I16x8LeU)),
        BYTE_PREFIX_I16x8_GE_S => Ok((bytes, InstructionType::I16x8GeS)),
        BYTE_PREFIX_I16x8_GE_U => Ok((bytes, InstructionType::I16x8GeU)),
        BYTE_PREFIX_I32x4_EQ => Ok((bytes, InstructionType::I32x4Eq)),
        BYTE_PREFIX_I32x4_NE => Ok((bytes, InstructionType::I32x4Ne)),
        BYTE_PREFIX_I32x4_LT_S => Ok((bytes, InstructionType::I32x4LtS)),
        BYTE_PREFIX_I32x4_LT_U => Ok((bytes, InstructionType::I32x4LtU)),
        BYTE_PREFIX_I32x4_GT_S => Ok((bytes, InstructionType::I32x4GtS)),
        BYTE_PREFIX_I32x4_GT_U => Ok((bytes, InstructionType::I32x4GtU)),
        BYTE_PREFIX_I32x4_LE_S => Ok((bytes, InstructionType::I32x4LeS)),
        BYTE_PREFIX_I32x4_LE_U => Ok((bytes, InstructionType::I32x4LeU)),
        BYTE_PREFIX_I32x4_GE_S => Ok((bytes, InstructionType::I32x4GeS)),
        BYTE_PREFIX_I32x4_GE_U => Ok((bytes, InstructionType::I32x4GeU)),
        BYTE_PREFIX_I64x2_EQ => Ok((bytes, InstructionType::I64x2Eq)),
        BYTE_PREFIX_I64x2_NE => Ok((bytes, InstructionType::I64x2Ne)),
        BYTE_PREFIX_I64x2_LT_S => Ok((bytes, InstructionType::I64x2LtS)),
        BYTE_PREFIX_I64x2_GT_S => Ok((bytes, InstructionType::I64x2GtS)),
        BYTE_PREFIX_I64x2_LE_S => Ok((bytes, InstructionType::I64x2LeS)),
        BYTE_PREFIX_I64x2_GE_S => Ok((bytes, InstructionType::I64x2GeS)),
        BYTE_PREFIX_F32x4_EQ => Ok((bytes, InstructionType::F32x4Eq)),
        BYTE_PREFIX_F32x4_NE => Ok((bytes, InstructionType::F32x4Ne)),
        BYTE_PREFIX_F32x4_LT => Ok((bytes, InstructionType::F32x4Lt)),
        BYTE_PREFIX_F32x4_GT => Ok((bytes, InstructionType::F32x4Gt)),
        BYTE_PREFIX_F32x4_LE => Ok((bytes, InstructionType::F32x4Le)),
        BYTE_PREFIX_F32x4_GE => Ok((bytes, InstructionType::F32x4Ge)),
        BYTE_PREFIX_F64x2_EQ => Ok((bytes, InstructionType::F64x2Eq)),
        BYTE_PREFIX_F64x2_NE => Ok((bytes, InstructionType::F64x2Ne)),
        BYTE_PREFIX_F64x2_LT => Ok((bytes, InstructionType::F64x2Lt)),
        BYTE_PREFIX_F64x2_GT => Ok((bytes, InstructionType::F64x2Gt)),
        BYTE_PREFIX_F64x2_LE => Ok((bytes, InstructionType::F64x2Le)),
        BYTE_PREFIX_F64x2_GE => Ok((bytes, InstructionType::F64x2Ge)),
        BYTE_PREFIX_V128_NOT => Ok((bytes, InstructionType::V128Not)),
        BYTE_PREFIX_V128_AND => Ok((bytes, InstructionType::V128And)),
        BYTE_PREFIX_V128_ANDNOT => Ok((bytes, InstructionType::V128AndNot)),
        BYTE_PREFIX_V128_OR => Ok((bytes, InstructionType::V128Or)),
        BYTE_PREFIX_V128_XOR => Ok((bytes, InstructionType::V128Xor)),
        BYTE_PREFIX_V128_BITSELECT => Ok((bytes, InstructionType::V128Bitselect)),
        BYTE_PREFIX_V128_ANYTRUE => Ok((bytes, InstructionType::V128AnyTrue)),
        BYTE_PREFIX_I8x16_ABS => Ok((bytes, InstructionType::I8x16Abs)),
        BYTE_PREFIX_I8x16_NEG => Ok((bytes, InstructionType::I8x16Neg)),
        BYTE_PREFIX_I8x16_POPCNT => Ok((bytes, InstructionType::I8x16Popcnt)),
        BYTE_PREFIX_I8x16_ALL_TRUE => Ok((bytes, InstructionType::I8x16AllTrue)),
        BYTE_PREFIX_I8x16_BITMASK => Ok((bytes, InstructionType::I8x16Bitmask)),
        BYTE_PREFIX_I8x16_NARROW_I16x8_S => Ok((bytes, InstructionType::I8x16NarrowI16x8S)),
        BYTE_PREFIX_I8x16_NARROW_I16x8_U => Ok((bytes, InstructionType::I8x16NarrowI16x8U)),
        BYTE_PREFIX_I8x16_SHL => Ok((bytes, InstructionType::I8x16Shl)),
        BYTE_PREFIX_I8x16_SHR_S => Ok((bytes, InstructionType::I8x16ShrS)),
        BYTE_PREFIX_I8x16_SHR_U => Ok((bytes, InstructionType::I8x16ShrU)),
        BYTE_PREFIX_I8x16_ADD => Ok((bytes, InstructionType::I8x16Add)),
        BYTE_PREFIX_I8x16_ADD_SAT_S => Ok((bytes, InstructionType::I8x16AddSatS)),
        BYTE_PREFIX_I8x16_ADD_SAT_U => Ok((bytes, InstructionType::I8x16AddSatU)),
        BYTE_PREFIX_I8x16_SUB => Ok((bytes, InstructionType::I8x16Sub)),
        BYTE_PREFIX_I8x16_SUB_SAT_S => Ok((bytes, InstructionType::I8x16SubSatS)),
        BYTE_PREFIX_I8x16_SUB_SAT_U => Ok((bytes, InstructionType::I8x16SubSatU)),
        BYTE_PREFIX_I8x16_MIN_S => Ok((bytes, InstructionType::I8x16MinS)),
        BYTE_PREFIX_I8x16_MIN_U => Ok((bytes, InstructionType::I8x16MinU)),
        BYTE_PREFIX_I8x16_MAX_S => Ok((bytes, InstructionType::I8x16MaxS)),
        BYTE_PREFIX_I8x16_MAX_U => Ok((bytes, InstructionType::I8x16MaxU)),
        BYTE_PREFIX_I8x16_AVGR_U => Ok((bytes, InstructionType::I8x16AvgrU)),
        BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_S => {
            Ok((bytes, InstructionType::I16x8ExtaddPairwiseI8x16S))
        }
        BYTE_PREFIX_I16x8_EXTADD_PAIRWISE_I8x16_U => {
            Ok((bytes, InstructionType::I16x8ExtaddPairwiseI8x16U))
        }
        BYTE_PREFIX_I16x8_ABS => Ok((bytes, InstructionType::I16x8Abs)),
        BYTE_PREFIX_I16x8_NEG => Ok((bytes, InstructionType::I16x8Neg)),
        BYTE_PREFIX_I16x8_Q15MULR_SAT_S => Ok((bytes, InstructionType::I16x8Q15MulrSatS)),
        BYTE_PREFIX_I16x8_ALL_TRUE => Ok((bytes, InstructionType::I16x8AllTrue)),
        BYTE_PREFIX_I16x8_BITMASK => Ok((bytes, InstructionType::I16x8Bitmask)),
        BYTE_PREFIX_I16x8_NARROW_I32x4_S => Ok((bytes, InstructionType::I16x8NarrowI32x4S)),
        BYTE_PREFIX_I16x8_NARROW_I32x4_U => Ok((bytes, InstructionType::I16x8NarrowI32x4U)),
        BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_S => Ok((bytes, InstructionType::I16x8ExtendLowI8x16S)),
        BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_S => {
            Ok((bytes, InstructionType::I16x8ExtendHighI8x16S))
        }
        BYTE_PREFIX_I16x8_EXTEND_LOW_I8x16_U => Ok((bytes, InstructionType::I16x8ExtendLowI8x16U)),
        BYTE_PREFIX_I16x8_EXTEND_HIGH_I8x16_U => {
            Ok((bytes, InstructionType::I16x8ExtendHighI8x16U))
        }
        BYTE_PREFIX_I16x8_SHL => Ok((bytes, InstructionType::I16x8Shl)),
        BYTE_PREFIX_I16x8_SHR_S => Ok((bytes, InstructionType::I16x8ShrS)),
        BYTE_PREFIX_I16x8_SHR_U => Ok((bytes, InstructionType::I16x8ShrU)),
        BYTE_PREFIX_I16x8_ADD => Ok((bytes, InstructionType::I16x8Add)),
        BYTE_PREFIX_I16x8_ADD_SAT_S => Ok((bytes, InstructionType::I16x8AddSatS)),
        BYTE_PREFIX_I16x8_ADD_SAT_U => Ok((bytes, InstructionType::I16x8AddSatU)),
        BYTE_PREFIX_I16x8_SUB => Ok((bytes, InstructionType::I16x8Sub)),
        BYTE_PREFIX_I16x8_SUB_SAT_S => Ok((bytes, InstructionType::I16x8SubSatS)),
        BYTE_PREFIX_I16x8_SUB_SAT_U => Ok((bytes, InstructionType::I16x8SubSatU)),
        BYTE_PREFIX_I16x8_MUL => Ok((bytes, InstructionType::I16x8Mul)),
        BYTE_PREFIX_I16x8_MIN_S => Ok((bytes, InstructionType::I16x8MinS)),
        BYTE_PREFIX_I16x8_MIN_U => Ok((bytes, InstructionType::I16x8MinU)),
        BYTE_PREFIX_I16x8_MAX_S => Ok((bytes, InstructionType::I16x8MaxS)),
        BYTE_PREFIX_I16x8_MAX_U => Ok((bytes, InstructionType::I16x8MaxU)),
        BYTE_PREFIX_I16x8_AVGR_U => Ok((bytes, InstructionType::I16x8AvgrU)),
        BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_S => Ok((bytes, InstructionType::I16x8ExtmulLowI8x16S)),
        BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_S => {
            Ok((bytes, InstructionType::I16x8ExtmulHighI8x16S))
        }
        BYTE_PREFIX_I16x8_EXTMUL_LOW_I8x16_U => Ok((bytes, InstructionType::I16x8ExtmulLowI8x16U)),
        BYTE_PREFIX_I16x8_EXTMUL_HIGH_I8x16_U => {
            Ok((bytes, InstructionType::I16x8ExtmulHighI8x16U))
        }
        BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_S => {
            Ok((bytes, InstructionType::I32x4ExtaddPairwiseI16x8S))
        }
        BYTE_PREFIX_I32x4_EXTADD_PAIRWISE_I16x8_U => {
            Ok((bytes, InstructionType::I32x4ExtaddPairwiseI16x8U))
        }
        BYTE_PREFIX_I32x4_ABS => Ok((bytes, InstructionType::I32x4Abs)),
        BYTE_PREFIX_I32x4_NEG => Ok((bytes, InstructionType::I32x4Neg)),
        BYTE_PREFIX_I32x4_ALL_TRUE => Ok((bytes, InstructionType::I32x4AllTrue)),
        BYTE_PREFIX_I32x4_BITMASK => Ok((bytes, InstructionType::I32x4Bitmask)),
        BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_S => Ok((bytes, InstructionType::I32x4ExtendLowI16x8S)),
        BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_S => {
            Ok((bytes, InstructionType::I32x4ExtendHighI16x8S))
        }
        BYTE_PREFIX_I32x4_EXTEND_LOW_I16x8_U => Ok((bytes, InstructionType::I32x4ExtendLowI16x8U)),
        BYTE_PREFIX_I32x4_EXTEND_HIGH_I16x8_U => {
            Ok((bytes, InstructionType::I32x4ExtendHighI16x8U))
        }
        BYTE_PREFIX_I32x4_SHL => Ok((bytes, InstructionType::I32x4Shl)),
        BYTE_PREFIX_I32x4_SHR_S => Ok((bytes, InstructionType::I32x4ShrS)),
        BYTE_PREFIX_I32x4_SHR_U => Ok((bytes, InstructionType::I32x4ShrU)),
        BYTE_PREFIX_I32x4_ADD => Ok((bytes, InstructionType::I32x4Add)),
        BYTE_PREFIX_I32x4_SUB => Ok((bytes, InstructionType::I32x4Sub)),
        BYTE_PREFIX_I32x4_MUL => Ok((bytes, InstructionType::I32x4Mul)),
        BYTE_PREFIX_I32x4_MIN_S => Ok((bytes, InstructionType::I32x4MinS)),
        BYTE_PREFIX_I32x4_MIN_U => Ok((bytes, InstructionType::I32x4MinU)),
        BYTE_PREFIX_I32x4_MAX_S => Ok((bytes, InstructionType::I32x4MaxS)),
        BYTE_PREFIX_I32x4_MAX_U => Ok((bytes, InstructionType::I32x4MaxU)),
        BYTE_PREFIX_I32x4_DOT_I16x8_S => Ok((bytes, InstructionType::I32x4DotI16x8S)),
        BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_S => Ok((bytes, InstructionType::I32x4ExtmulLowI16x8S)),
        BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_S => {
            Ok((bytes, InstructionType::I32x4ExtmulHighI16x8S))
        }
        BYTE_PREFIX_I32x4_EXTMUL_LOW_I16x8_U => Ok((bytes, InstructionType::I32x4ExtmulLowI16x8U)),
        BYTE_PREFIX_I32x4_EXTMUL_HIGH_I16x8_U => {
            Ok((bytes, InstructionType::I32x4ExtmulHighI16x8U))
        }
        BYTE_PREFIX_I64x2_ABS => Ok((bytes, InstructionType::I64x2Abs)),
        BYTE_PREFIX_I64x2_NEG => Ok((bytes, InstructionType::I64x2Neg)),
        BYTE_PREFIX_I64x2_ALL_TRUE => Ok((bytes, InstructionType::I64x2AllTrue)),
        BYTE_PREFIX_I64x2_BITMASK => Ok((bytes, InstructionType::I64x2Bitmask)),
        BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_S => Ok((bytes, InstructionType::I64x2ExtendLowI32x4S)),
        BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_S => {
            Ok((bytes, InstructionType::I64x2ExtendHighI32x4S))
        }
        BYTE_PREFIX_I64x2_EXTEND_LOW_I32x4_U => Ok((bytes, InstructionType::I64x2ExtendLowI32x4U)),
        BYTE_PREFIX_I64x2_EXTEND_HIGH_I32x4_U => {
            Ok((bytes, InstructionType::I64x2ExtendHighI32x4U))
        }
        BYTE_PREFIX_I64x2_SHL => Ok((bytes, InstructionType::I64x2Shl)),
        BYTE_PREFIX_I64x2_SHR_S => Ok((bytes, InstructionType::I64x2ShrS)),
        BYTE_PREFIX_I64x2_SHR_U => Ok((bytes, InstructionType::I64x2ShrU)),
        BYTE_PREFIX_I64x2_ADD => Ok((bytes, InstructionType::I64x2Add)),
        BYTE_PREFIX_I64x2_SUB => Ok((bytes, InstructionType::I64x2Sub)),
        BYTE_PREFIX_I64x2_MUL => Ok((bytes, InstructionType::I64x2Mul)),
        BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_S => Ok((bytes, InstructionType::I64x2ExtmulLowI32x4S)),
        BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_S => {
            Ok((bytes, InstructionType::I64x2ExtmulHighI32x4S))
        }
        BYTE_PREFIX_I64x2_EXTMUL_LOW_I32x4_U => Ok((bytes, InstructionType::I64x2ExtmulLowI32x4U)),
        BYTE_PREFIX_I64x2_EXTMUL_HIGH_I32x4_U => {
            Ok((bytes, InstructionType::I64x2ExtmulHighI32x4U))
        }
        BYTE_PREFIX_F32x4_CEIL => Ok((bytes, InstructionType::F32x4Ceil)),
        BYTE_PREFIX_F32x4_FLOOR => Ok((bytes, InstructionType::F32x4Floor)),
        BYTE_PREFIX_F32x4_TRUNC => Ok((bytes, InstructionType::F32x4Trunc)),
        BYTE_PREFIX_F32x4_NEAREST => Ok((bytes, InstructionType::F32x4Nearest)),
        BYTE_PREFIX_F32x4_ABS => Ok((bytes, InstructionType::F32x4Abs)),
        BYTE_PREFIX_F32x4_NEG => Ok((bytes, InstructionType::F32x4Neg)),
        BYTE_PREFIX_F32x4_SQRT => Ok((bytes, InstructionType::F32x4Sqrt)),
        BYTE_PREFIX_F32x4_ADD => Ok((bytes, InstructionType::F32x4Add)),
        BYTE_PREFIX_F32x4_SUB => Ok((bytes, InstructionType::F32x4Sub)),
        BYTE_PREFIX_F32x4_MUL => Ok((bytes, InstructionType::F32x4Mul)),
        BYTE_PREFIX_F32x4_DIV => Ok((bytes, InstructionType::F32x4Div)),
        BYTE_PREFIX_F32x4_MIN => Ok((bytes, InstructionType::F32x4Min)),
        BYTE_PREFIX_F32x4_MAX => Ok((bytes, InstructionType::F32x4Max)),
        BYTE_PREFIX_F32x4_PMIN => Ok((bytes, InstructionType::F32x4Pmin)),
        BYTE_PREFIX_F32x4_PMAX => Ok((bytes, InstructionType::F32x4Pmax)),
        BYTE_PREFIX_F64x2_CEIL => Ok((bytes, InstructionType::F64x2Ceil)),
        BYTE_PREFIX_F64x2_FLOOR => Ok((bytes, InstructionType::F64x2Floor)),
        BYTE_PREFIX_F64x2_TRUNC => Ok((bytes, InstructionType::F64x2Trunc)),
        BYTE_PREFIX_F64x2_NEAREST => Ok((bytes, InstructionType::F64x2Nearest)),
        BYTE_PREFIX_F64x2_ABS => Ok((bytes, InstructionType::F64x2Abs)),
        BYTE_PREFIX_F64x2_NEG => Ok((bytes, InstructionType::F64x2Neg)),
        BYTE_PREFIX_F64x2_SQRT => Ok((bytes, InstructionType::F64x2Sqrt)),
        BYTE_PREFIX_F64x2_ADD => Ok((bytes, InstructionType::F64x2Add)),
        BYTE_PREFIX_F64x2_SUB => Ok((bytes, InstructionType::F64x2Sub)),
        BYTE_PREFIX_F64x2_MUL => Ok((bytes, InstructionType::F64x2Mul)),
        BYTE_PREFIX_F64x2_DIV => Ok((bytes, InstructionType::F64x2Div)),
        BYTE_PREFIX_F64x2_MIN => Ok((bytes, InstructionType::F64x2Min)),
        BYTE_PREFIX_F64x2_MAX => Ok((bytes, InstructionType::F64x2Max)),
        BYTE_PREFIX_F64x2_PMIN => Ok((bytes, InstructionType::F64x2Pmin)),
        BYTE_PREFIX_F64x2_PMAX => Ok((bytes, InstructionType::F64x2Pmax)),
        BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_S => Ok((bytes, InstructionType::I32x4TruncSatF32x4S)),
        BYTE_PREFIX_I32x4_TRUNC_SAT_F32x4_U => Ok((bytes, InstructionType::I32x4TruncSatF32x4U)),
        BYTE_PREFIX_F32x4_CONVERT_I32x4_S => Ok((bytes, InstructionType::F32x4ConvertI32x4S)),
        BYTE_PREFIX_F32x4_CONVERT_I32x4_U => Ok((bytes, InstructionType::F32x4ConvertI32x4U)),
        BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_S_ZERO => {
            Ok((bytes, InstructionType::I32x4TruncSatF64x2SZero))
        }
        BYTE_PREFIX_I32x4_TRUNC_SAT_F64x2_U_ZERO => {
            Ok((bytes, InstructionType::I32x4TruncSatF64x2UZero))
        }
        BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_S => {
            Ok((bytes, InstructionType::F64x2ConvertLowI32x4S))
        }
        BYTE_PREFIX_F64x2_CONVERT_LOW_I32x4_U => {
            Ok((bytes, InstructionType::F64x2ConvertLowI32x4U))
        }
        BYTE_PREFIX_F32x4_DEMOTE_F64x2_ZERO => Ok((bytes, InstructionType::F32x4DemoteF64x2Zero)),
        BYTE_PREFIX_F64x2_PROMOTE_LOW_F32x4 => Ok((bytes, InstructionType::F64x2PromoteLowF32x4)),

        _ => Err(nom::Err::Failure(nom::error::Error::new(
            bytes,
            nom::error::ErrorKind::Fail,
        ))),
    }
}
