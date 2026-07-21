/ test.q

/ Dynamically resolve library path relative to this script
path: string .z.f;
dir: $[count "/" vs path; "/" sv -1_ "/" vs path; "."];
if[dir ~ ""; dir: "."];
lib: hsym `$(dir, "/../target/release/examples/libk4rust_test_ffi");

add_vectors:    lib 2: (`add_vectors; 2)
scale_floats:   lib 2: (`scale_floats; 2)
filter_greater: lib 2: (`filter_greater; 2)
count_char:     lib 2: (`count_char; 2)
sum_mixed:      lib 2: (`sum_mixed; 2)
dot_product:    lib 2: (`dot_product; 2)
filter_mask:    lib 2: (`filter_mask; 2)
count_symbol:   lib 2: (`count_symbol; 2)
add_mixed:      lib 2: (`add_mixed; 2)
generate_vector: lib 2: (`generate_vector; 1)
package_results: lib 2: (`package_results; 2)
test_b9:            lib 2: (`test_b9; 2)
test_d9:            lib 2: (`test_d9; 1)
test_leak_krr:      lib 2: (`test_leak_krr; 1)
trigger_panic:      lib 2: (`trigger_panic; 1)
test_new_apis:      lib 2: (`test_new_apis; 1)
test_sd_setup:      lib 2: (`test_sd_setup; 1)
test_sd_write:      lib 2: (`test_sd_write; 2)
test_sd_status:     lib 2: (`test_sd_status; 1)
test_sd_close:      lib 2: (`test_sd_close; 2)

/ Helper to format output
info:{[msg] -1 msg;}

/ Helper to run and assert
assert:{[name;expected;actual]
    $[expected ~ actual;
        info "[PASS] ",name;
        [info "[FAIL] ",name," - Expected: ",(.Q.s expected)," Got: ",(.Q.s actual); exit 1]
    ];
 }

/ --- 1. Test add_vectors ---
info "Running add_vectors tests...";
assert["add_vectors normal"; 2 4 6 0N 0N; add_vectors[1 2 3 0N 5; 1 2 3 4 0N]]
assert["add_vectors empty"; 0#0j; add_vectors[0#0j; 0#0j]]
assert["add_vectors type mismatch"; "type"; .[add_vectors; (1 2 3; 1.0 2.0 3.0); {x}]]

/ --- 2. Test scale_floats ---
info "Running scale_floats tests...";
assert["scale_floats normal"; 2.5 5.0 7.5; scale_floats[1.0 2.0 3.0; 2.5]]
assert["scale_floats empty"; 0#0.0; scale_floats[0#0.0; 2.5]]
assert["scale_floats type mismatch"; "type"; .[scale_floats; (1 2 3; 2.5); {x}]]

/ --- 3. Test filter_greater ---
info "Running filter_greater tests...";
assert["filter_greater normal"; 3 4 5i; filter_greater[1 2 3 4 5i; 2i]]
assert["filter_greater empty"; 0#0i; filter_greater[0#0i; 2i]]
assert["filter_greater type mismatch"; "type"; .[filter_greater; (1 2 3i; 2.5); {x}]]

/ --- 4. Test count_char ---
info "Running count_char tests...";
assert["count_char normal"; 5i; count_char["abracadabra"; "a"]]
assert["count_char empty"; 0i; count_char[""; "a"]]
assert["count_char type mismatch"; "type"; .[count_char; ("abracadabra"; 1i); {x}]]

/ --- 5. Test sum_mixed ---
info "Running sum_mixed tests...";
mixed_list: (1 2 3; 10 20 30 40; 100 200)
assert["sum_mixed 0"; 6j; sum_mixed[mixed_list; 0i]]
assert["sum_mixed 1"; 100j; sum_mixed[mixed_list; 1i]]
assert["sum_mixed 2"; 300j; sum_mixed[mixed_list; 2i]]
assert["sum_mixed type mismatch"; "type"; .[sum_mixed; (mixed_list; 2.5); {x}]]

/ --- 6. Test dot_product ---
info "Running dot_product tests...";
assert["dot_product normal"; 32.0; dot_product[1.0 2.0 3.0; 4.0 5.0 6.0]]
assert["dot_product empty"; 0.0; dot_product[0#0.0; 0#0.0]]
assert["dot_product type mismatch"; "type"; .[dot_product; (1.0 2.0; 1 2); {x}]]
assert["dot_product length mismatch"; "length"; .[dot_product; (1.0 2.0; 1.0 2.0 3.0); {x}]]

/ --- 7. Test filter_mask ---
info "Running filter_mask tests...";
assert["filter_mask normal"; 2 4j; filter_mask[1 2 3 4j; 0101b]]
assert["filter_mask empty"; 0#0j; filter_mask[0#0j; 0#0b]]
assert["filter_mask type mismatch"; "type"; .[filter_mask; (1 2 3; 0 1 0); {x}]]
assert["filter_mask length mismatch"; "length"; .[filter_mask; (1 2 3j; 01b); {x}]]

/ --- 8. Test count_symbol ---
info "Running count_symbol tests...";
assert["count_symbol normal"; 2i; count_symbol[`a`b`a`c; `a]]
assert["count_symbol empty"; 0i; count_symbol[0#`; `a]]
assert["count_symbol type mismatch"; "type"; .[count_symbol; (`a`b; "a"); {x}]]

/ --- 9. Test add_mixed ---
info "Running add_mixed tests...";
assert["add_mixed long+float"; 11.0 22.0 33.0; add_mixed[1 2 3j; 10 20 30.0]]
assert["add_mixed int+long"; 11 22 33j; add_mixed[1 2 3i; 10 20 30j]]
assert["add_mixed short+byte"; 11 13 15h; add_mixed[1 2 3h; 0x0a0b0c]]
assert["add_mixed long null + float"; 11.0 22.0 0Nf; add_mixed[1 2 0Nj; 10 20 30.0]]
assert["add_mixed float null + long"; 11.0 22.0 0Nf; add_mixed[1 2 0Nf; 10 20 30j]]
assert["add_mixed int null + long"; 11 22 0Nj; add_mixed[1 2 0Ni; 10 20 30j]]

/ --- 10. Test generate_vector ---
info "Running generate_vector tests...";
assert["generate_vector normal int"; 0 1 2 3 4j; generate_vector[5i]]
assert["generate_vector normal long"; 0 1 2 3 4j; generate_vector[5j]]
assert["generate_vector Q math"; 10j; sum generate_vector[5]]
assert["generate_vector type mismatch"; "type"; .[generate_vector; (enlist 5.0); {x}]]

/ --- 11. Test package_results ---
info "Running package_results tests...";
x_in: 1 2 3j;
y_in: 10.0 20.0 30.0;
res: package_results[x_in; y_in];
assert["package_results x"; x_in; res[0]];
assert["package_results y"; y_in; res[1]];
delete res from `.;
.Q.gc[];
assert["package_results safety x"; 1 2 3j; x_in];
assert["package_results safety y"; 10.0 20.0 30.0; y_in];
delete x_in, y_in from `.;

/ --- 12. Test b9/d9 serialization ---
info "Running serialization (b9/d9) tests...";
test_dict: `a`b`c!10 20 30j;
test_table: flip `sym`price!(`apple`google; 150.5 2800.2);

/ Check b9 matches -8!
assert["b9 matches -8! dict"; -8!test_dict; test_b9[0i; test_dict]];
assert["b9 matches -8! table"; -8!test_table; test_b9[0i; test_table]];

/ Check d9 matches -9!
ser_dict: -8!test_dict;
ser_table: -8!test_table;
assert["d9 matches -9! dict"; -9! ser_dict; test_d9[ser_dict]];
assert["d9 matches -9! table"; -9! ser_table; test_d9[ser_table]];

delete test_dict, test_table, ser_dict, ser_table from `.;

/ --- 13. Test error / panic catching & new API execution ---
info "Running error, panic, and new APIs FFI tests...";
assert["test_leak_krr error handling"; "test_krr_error"; .[test_leak_krr; enlist 100j; {x}]];
assert["trigger_panic panic catching"; "panic"; .[trigger_panic; enlist (::); {x}]];
assert["test_new_apis executes cleanly"; 1 2 3j; test_new_apis[1 2 3j]];

/ --- 14. Test event loop registration (sd1, sd0, sd0x) ---
info "Running event loop (sd1/sd0/sd0x) tests...";
system "p 50002"; / Open port to force kdb+ to run active socket polling
fds: test_sd_setup[(::)];

/ Initial status: called_count should be 0, data should be empty
status: test_sd_status[(::)];
assert["sd1 initial count"; 0i; status[0]];
assert["sd1 initial data"; ""; status[1]];

/ Write data to fd2 (connected to fd1 monitored by kdb+)
write_ok: test_sd_write[fds[1]; "event_loop_payload"];
assert["sd write success"; 1b; write_ok];

/ Set a timer to trigger event loop processing and execute assertions
system "t 50";
called_ticks: 0;
.z.ts:{
    called_ticks:: called_ticks + 1;
    status: test_sd_status[(::)];
    if[status[0] > 0;
        [
            system "t 0"; / turn off timer
            system "p 0"; / turn off port
            info "[PASS] sd1 processed callback count";
            assert["sd1 processed data"; "event_loop_payload"; status[1]];
            
            / Unregister fd1 using sd0x (mode 1)
            close_ok: test_sd_close[fds[0]; 1i];
            assert["sd0x close success"; 1b; close_ok];

            / Close fd2 using sd0 and manual close (mode 0)
            close_ok2: test_sd_close[fds[1]; 0i];
            assert["sd0 close success"; 1b; close_ok2];

            delete fds, status, write_ok, close_ok, close_ok2, called_ticks from `.;
            info "All tests completed successfully!";
            exit 0;
        ]];
    if[called_ticks > 40; / Timeout after 2 seconds (40 * 50ms)
        [
            system "t 0";
            system "p 0";
            info "[FAIL] sd1 event loop did not trigger within timeout";
            exit 1;
        ]];
 };
