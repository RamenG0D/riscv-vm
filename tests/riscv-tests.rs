use riscv_vm::{
    bus::VirtualDevice,
    cpu::{Cpu, Privilege, Riscv32Cpu},
    memory::dram::{Dram, DRAM_BASE, DRAM_SIZE},
    trap::Trap,
};

macro_rules! add_test {
    ($name:ident) => {
        #[test]
        fn $name() -> ::std::io::Result<()> {
            riscv_vm::init_logging(riscv_vm::log::LevelFilter::Debug);

            let data = include_bytes!(concat!(
                concat!(
                    concat!(env!("CARGO_MANIFEST_DIR"), "/tests/rvtests/"),
                    stringify!($name)
                ),
                ".bin"
            ));
            let mut cpu = Riscv32Cpu::new();
            let dram = {
                let mut tmp = Box::new(Dram::new());
                tmp.initialize(data);
                tmp
            };
            cpu.add_device(VirtualDevice::new(dram, DRAM_BASE, DRAM_SIZE));
            cpu.register_syscall(93, |cpu| {
                let exit_code = *cpu.get_register(10 /* register ( a0 ) */).unwrap();
                let gp = *cpu.get_register(3 /* register ( gp ) */).unwrap();
                cpu.set_privilege(Privilege::Machine);

                if exit_code != 0 || gp != 1 {
                    panic!(
                        "exit code: {0:} [{0:#X}], gp: {1:} [{1:#X}], pc: {2:} [{2:#X}]",
                        exit_code,
                        gp,
                        cpu.get_pc()
                    );
                }

                Trap::Fatal
            });

            let _ = cpu.run();

            let a0 = cpu.get_register(10).unwrap();
            assert_eq!(*a0, 0);

            let mode = cpu.get_privilege();
            assert_eq!(mode, Privilege::Machine);

            Ok(())
        }
    };
}

/*
add_test!(rv32mi_p_breakpoint);
add_test!(rv32mi_p_csr);
add_test!(rv32mi_p_illegal);
add_test!(rv32mi_p_lh_misaligned);
add_test!(rv32mi_p_lw_misaligned);
add_test!(rv32mi_p_ma_addr);
add_test!(rv32mi_p_ma_fetch);
add_test!(rv32mi_p_mcsr);
add_test!(rv32mi_p_sbreak);
add_test!(rv32mi_p_scall);
add_test!(rv32mi_p_shamt);
add_test!(rv32mi_p_sh_misaligned);
add_test!(rv32mi_p_sw_misaligned);
add_test!(rv32mi_p_zicntr);
add_test!(rv32si_p_csr);
add_test!(rv32si_p_dirty);
add_test!(rv32si_p_ma_fetch);
add_test!(rv32si_p_sbreak);
add_test!(rv32si_p_scall);
add_test!(rv32si_p_wfi);
add_test!(rv32ua_p_amoadd_w);
add_test!(rv32ua_p_amoand_w);
add_test!(rv32ua_p_amomaxu_w);
add_test!(rv32ua_p_amomax_w);
add_test!(rv32ua_p_amominu_w);
add_test!(rv32ua_p_amomin_w);
add_test!(rv32ua_p_amoor_w);
add_test!(rv32ua_p_amoswap_w);
add_test!(rv32ua_p_amoxor_w);
add_test!(rv32ua_p_lrsc);
add_test!(rv32ua_v_amoadd_w);
add_test!(rv32ua_v_amoand_w);
add_test!(rv32ua_v_amomaxu_w);
add_test!(rv32ua_v_amomax_w);
add_test!(rv32ua_v_amominu_w);
add_test!(rv32ua_v_amomin_w);
add_test!(rv32ua_v_amoor_w);
add_test!(rv32ua_v_amoswap_w);
add_test!(rv32ua_v_amoxor_w);
add_test!(rv32ua_v_lrsc);
add_test!(rv32uc_p_rvc);
add_test!(rv32uc_v_rvc);
add_test!(rv32ud_p_fadd);
add_test!(rv32ud_p_fclass);
add_test!(rv32ud_p_fcmp);
add_test!(rv32ud_p_fcvt);
add_test!(rv32ud_p_fcvt_w);
add_test!(rv32ud_p_fdiv);
add_test!(rv32ud_p_fmadd);
add_test!(rv32ud_p_fmin);
add_test!(rv32ud_p_ldst);
add_test!(rv32ud_p_recoding);
add_test!(rv32ud_v_fadd);
add_test!(rv32ud_v_fclass);
add_test!(rv32ud_v_fcmp);
add_test!(rv32ud_v_fcvt);
add_test!(rv32ud_v_fcvt_w);
add_test!(rv32ud_v_fdiv);
add_test!(rv32ud_v_fmadd);
add_test!(rv32ud_v_fmin);
add_test!(rv32ud_v_ldst);
add_test!(rv32ud_v_recoding);
add_test!(rv32uf_p_fadd);
add_test!(rv32uf_p_fclass);
add_test!(rv32uf_p_fcmp);
add_test!(rv32uf_p_fcvt);
add_test!(rv32uf_p_fcvt_w);
add_test!(rv32uf_p_fdiv);
add_test!(rv32uf_p_fmadd);
add_test!(rv32uf_p_fmin);
add_test!(rv32uf_p_ldst);
add_test!(rv32uf_p_move);
add_test!(rv32uf_p_recoding);
add_test!(rv32uf_v_fadd);
add_test!(rv32uf_v_fclass);
add_test!(rv32uf_v_fcmp);
add_test!(rv32uf_v_fcvt);
add_test!(rv32uf_v_fcvt_w);
add_test!(rv32uf_v_fdiv);
add_test!(rv32uf_v_fmadd);
add_test!(rv32uf_v_fmin);
add_test!(rv32uf_v_ldst);
add_test!(rv32uf_v_move);
add_test!(rv32uf_v_recoding);
*/

// User mode - physical addressing
add_test!(rv32ui_p_add);
add_test!(rv32ui_p_addi);
add_test!(rv32ui_p_and);
add_test!(rv32ui_p_andi);
add_test!(rv32ui_p_auipc);
add_test!(rv32ui_p_beq);
add_test!(rv32ui_p_bge);
add_test!(rv32ui_p_bgeu);
add_test!(rv32ui_p_blt);
add_test!(rv32ui_p_bltu);
add_test!(rv32ui_p_bne);
add_test!(rv32ui_p_fence_i);
add_test!(rv32ui_p_jal);
add_test!(rv32ui_p_jalr);
add_test!(rv32ui_p_lb);
add_test!(rv32ui_p_lbu);
add_test!(rv32ui_p_ld_st);
add_test!(rv32ui_p_lh);
add_test!(rv32ui_p_lhu);
add_test!(rv32ui_p_lui);
add_test!(rv32ui_p_lw);
add_test!(rv32ui_p_ma_data);
add_test!(rv32ui_p_or);
add_test!(rv32ui_p_ori);
add_test!(rv32ui_p_sb);
add_test!(rv32ui_p_sh);
add_test!(rv32ui_p_simple);
add_test!(rv32ui_p_sll);
add_test!(rv32ui_p_slli);
add_test!(rv32ui_p_slt);
add_test!(rv32ui_p_slti);
add_test!(rv32ui_p_sltiu);
add_test!(rv32ui_p_sltu);
add_test!(rv32ui_p_sra);
add_test!(rv32ui_p_srai);
add_test!(rv32ui_p_srl);
add_test!(rv32ui_p_srli);
add_test!(rv32ui_p_st_ld);
add_test!(rv32ui_p_sub);
add_test!(rv32ui_p_sw);
add_test!(rv32ui_p_xor);
add_test!(rv32ui_p_xori);

// User mode - virtual addressing
add_test!(rv32ui_v_add);
// add_test!(rv32ui_v_addi);
// add_test!(rv32ui_v_and);
// add_test!(rv32ui_v_andi);
// add_test!(rv32ui_v_auipc);
// add_test!(rv32ui_v_beq);
// add_test!(rv32ui_v_bge);
// add_test!(rv32ui_v_bgeu);
// add_test!(rv32ui_v_blt);
// add_test!(rv32ui_v_bltu);
// add_test!(rv32ui_v_bne);
// add_test!(rv32ui_v_fence_i);
// add_test!(rv32ui_v_jal);
// add_test!(rv32ui_v_jalr);
// add_test!(rv32ui_v_lb);
// add_test!(rv32ui_v_lbu);
// add_test!(rv32ui_v_ld_st);
// add_test!(rv32ui_v_lh);
// add_test!(rv32ui_v_lhu);
// add_test!(rv32ui_v_lui);
// add_test!(rv32ui_v_lw);
// add_test!(rv32ui_v_ma_data);
// add_test!(rv32ui_v_or);
// add_test!(rv32ui_v_ori);
// add_test!(rv32ui_v_sb);
// add_test!(rv32ui_v_sh);
// add_test!(rv32ui_v_simple);
// add_test!(rv32ui_v_sll);
// add_test!(rv32ui_v_slli);
// add_test!(rv32ui_v_slt);
// add_test!(rv32ui_v_slti);
// add_test!(rv32ui_v_sltiu);
// add_test!(rv32ui_v_sltu);
// add_test!(rv32ui_v_sra);
// add_test!(rv32ui_v_srai);
// add_test!(rv32ui_v_srl);
// add_test!(rv32ui_v_srli);
// add_test!(rv32ui_v_st_ld);
// add_test!(rv32ui_v_sub);
// add_test!(rv32ui_v_sw);
// add_test!(rv32ui_v_xor);
// add_test!(rv32ui_v_xori);

// User mode - physical addressing - compressed
// add_test!(rv32um_p_div);
// add_test!(rv32um_p_divu);
// add_test!(rv32um_p_mul);
// add_test!(rv32um_p_mulh);
// add_test!(rv32um_p_mulhsu);
// add_test!(rv32um_p_mulhu);
// add_test!(rv32um_p_rem);
// add_test!(rv32um_p_remu);
// add_test!(rv32um_v_div);
// add_test!(rv32um_v_divu);
// add_test!(rv32um_v_mul);
// add_test!(rv32um_v_mulh);
// add_test!(rv32um_v_mulhsu);
// add_test!(rv32um_v_mulhu);
// add_test!(rv32um_v_rem);
// add_test!(rv32um_v_remu);
/*add_test!(rv32uzba_p_sh1add);
add_test!(rv32uzba_p_sh2add);
add_test!(rv32uzba_p_sh3add);
add_test!(rv32uzba_v_sh1add);
add_test!(rv32uzba_v_sh2add);
add_test!(rv32uzba_v_sh3add);
add_test!(rv32uzbb_p_andn);
add_test!(rv32uzbb_p_clz);
add_test!(rv32uzbb_p_cpop);
add_test!(rv32uzbb_p_ctz);
add_test!(rv32uzbb_p_max);
add_test!(rv32uzbb_p_maxu);
add_test!(rv32uzbb_p_min);
add_test!(rv32uzbb_p_minu);
add_test!(rv32uzbb_p_orc_b);
add_test!(rv32uzbb_p_orn);
add_test!(rv32uzbb_p_rev8);
add_test!(rv32uzbb_p_rol);
add_test!(rv32uzbb_p_ror);
add_test!(rv32uzbb_p_rori);
add_test!(rv32uzbb_p_sext_b);
add_test!(rv32uzbb_p_sext_h);
add_test!(rv32uzbb_p_xnor);
add_test!(rv32uzbb_p_zext_h);
add_test!(rv32uzbb_v_andn);
add_test!(rv32uzbb_v_clz);
add_test!(rv32uzbb_v_cpop);
add_test!(rv32uzbb_v_ctz);
add_test!(rv32uzbb_v_max);
add_test!(rv32uzbb_v_maxu);
add_test!(rv32uzbb_v_min);
add_test!(rv32uzbb_v_minu);
add_test!(rv32uzbb_v_orc_b);
add_test!(rv32uzbb_v_orn);
add_test!(rv32uzbb_v_rev8);
add_test!(rv32uzbb_v_rol);
add_test!(rv32uzbb_v_ror);
add_test!(rv32uzbb_v_rori);
add_test!(rv32uzbb_v_sext_b);
add_test!(rv32uzbb_v_sext_h);
add_test!(rv32uzbb_v_xnor);
add_test!(rv32uzbb_v_zext_h);
add_test!(rv32uzbc_p_clmul);
add_test!(rv32uzbc_p_clmulh);
add_test!(rv32uzbc_p_clmulr);
add_test!(rv32uzbc_v_clmul);
add_test!(rv32uzbc_v_clmulh);
add_test!(rv32uzbc_v_clmulr);
add_test!(rv32uzbs_p_bclr);
add_test!(rv32uzbs_p_bclri);
add_test!(rv32uzbs_p_bext);
add_test!(rv32uzbs_p_bexti);
add_test!(rv32uzbs_p_binv);
add_test!(rv32uzbs_p_binvi);
add_test!(rv32uzbs_p_bset);
add_test!(rv32uzbs_p_bseti);
add_test!(rv32uzbs_v_bclr);
add_test!(rv32uzbs_v_bclri);
add_test!(rv32uzbs_v_bext);
add_test!(rv32uzbs_v_bexti);
add_test!(rv32uzbs_v_binv);
add_test!(rv32uzbs_v_binvi);
add_test!(rv32uzbs_v_bset);
add_test!(rv32uzbs_v_bseti);
add_test!(rv32uzfh_p_fadd);
add_test!(rv32uzfh_p_fclass);
add_test!(rv32uzfh_p_fcmp);
add_test!(rv32uzfh_p_fcvt);
add_test!(rv32uzfh_p_fcvt_w);
add_test!(rv32uzfh_p_fdiv);
add_test!(rv32uzfh_p_fmadd);
add_test!(rv32uzfh_p_fmin);
add_test!(rv32uzfh_p_ldst);
add_test!(rv32uzfh_p_move);
add_test!(rv32uzfh_p_recoding);
add_test!(rv32uzfh_v_fadd);
add_test!(rv32uzfh_v_fclass);
add_test!(rv32uzfh_v_fcmp);
add_test!(rv32uzfh_v_fcvt);
add_test!(rv32uzfh_v_fcvt_w);
add_test!(rv32uzfh_v_fdiv);
add_test!(rv32uzfh_v_fmadd);
add_test!(rv32uzfh_v_fmin);
add_test!(rv32uzfh_v_ldst);
add_test!(rv32uzfh_v_move);
add_test!(rv32uzfh_v_recoding);*/
