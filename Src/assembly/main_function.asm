[2K            [31m[31m; CALL XREF from entry0 @ [31m0x140585047[31m[0m
[36m┌[0m 422: int [31mmain[0m (int argc, char **argv, char **envp);
[36m│[0m           [37m; [37mvar [34mint64_t var_58h [36m@ rbp-0x58[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_50h [36m@ rbp-0x50[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_48h [36m@ rbp-0x48[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_40h [36m@ rbp-0x40[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_38h [36m@ rbp-0x38[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_18h [36m@ rbp-0x18[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_9h [36m@ rbp-0x9[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_8h [36m@ rbp-0x8[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_20h [36m@ rsp+0x20[0m
[36m│[0m           [37m; [37mvar [34mint64_t var_80h [36m@ rsp+0x80[0m
[36m│[0m           [32m0x140126620[0m      [33m55[0m             [35mpush[36m rbp[0m[0m[0m
[36m│[0m           [32m0x140126621[0m      [33m48[37m81[37mec[37ma0[32m00[32m00[37m.[0m  [33msub[36m rsp[0m,[36m[36m [33m0xa0[0m[0m[0m
[36m│[0m           [32m0x140126628[0m      [33m48[37m8d[37mac[33m24[37m80[32m00[37m.[0m  [37mlea[36m rbp[0m,[36m[36m [0m[[34mvar_80h[0m][36m[0m[0m[0m
[36m│[0m           [32m0x140126630[0m      [33m48[37mc7[33m45[37mf8[37mfe[31mff[37m.[0m  [37mmov qword[36m [0m[[34mvar_8h[0m][36m[0m,[36m[36m [33m0xfffffffffffffffe[0m[0m[0m
[36m│[0m           [32m0x140126638[0m      [33m48[37m8d[37m15[37m91[33m4e[33m29[37m.[0m  [37mlea[36m rdx[0m,[36m[36m [0m[[36m[33m0x1403bb4d0[0m][36m[0m[0m[0m
[36m│[0m           [32m0x14012663f[0m      [33m31[37mc9[0m           [33mxor[36m ecx[0m,[36m[36m ecx[0m[0m[0m
[36m│[0m           [32m0x140126641[0m      [31mff[37m15[37m81[37mba[33m48[32m00[0m   [32mcall qword [sym.imp.kernel32.dll_AddVectoredExceptionHandler][0m[31m [31m; [[31m0x1405b20c8[31m:8]=0x7bdb14 reloc.kernel32.dll_AddVectoredExceptionHandler[0m
[36m│[0m           [32m0x140126647[0m      [33m48[37m85[37mc0[0m         [33mtest[36m rax[0m,[36m[36m rax[0m[0m[0m
[36m│[0m       [36m┌[0m[36m─[0m[36m<[0m [32m0x14012664a[0m      [37m0f[37m84[37mf7[32m00[32m00[32m00[0m   [32mje 0x140126747[0m[0m
[36m│[0m       [36m│[0m   [32m0x140126650[0m      [37mc7[33m45[37mb0[32m00[33m50[32m00[37m.[0m  [37mmov dword[36m [0m[[34mvar_50h[0m][36m[0m,[36m[36m [33m0x5000[0m[0m[0m
[36m│[0m       [36m│[0m   [32m0x140126657[0m      [33m48[37m8d[33m4d[37mb0[0m       [37mlea[36m rcx[0m,[36m[36m [0m[[34mvar_50h[0m][36m[0m[0m[0m
[36m│[0m       [36m│[0m   [32m0x14012665b[0m      [31mff[37m15[33m6f[37mba[33m48[32m00[0m   [32mcall qword [sym.imp.kernel32.dll_SetThreadStackGuarantee][0m[31m [31m; [[31m0x1405b20d0[31m:8]=0x7bdb32 reloc.kernel32.dll_SetThreadStackGuarantee[31m [31m; "2\xdb{"[0m
[36m│[0m       [36m│[0m   [32m0x140126661[0m      [37m85[37mc0[0m           [33mtest[36m eax[0m,[36m[36m eax[0m[0m[0m
[36m│[0m      [36m┌[0m[36m─[0m[36m─[0m[36m<[0m [32m0x140126663[0m      [33m75[37m0f[0m           [32mjne 0x140126674[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x140126665[0m      [31mff[37m15[37m9d[37mbb[33m48[32m00[0m   [32mcall qword [sym.imp.kernel32.dll_GetLastError][0m[31m [31m; [[31m0x1405b2208[31m:8]=0x7bda4a reloc.kernel32.dll_GetLastError[31m [31m; "J\xda{"[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x14012666b[0m      [37m83[37mf8[33m78[0m         [33mcmp[36m eax[0m,[36m[36m [33m0x78[0m[0m[31m              [31m; 120[0m
[36m│[0m     [36m┌[0m[36m─[0m[36m─[0m[36m─[0m[36m<[0m [32m0x14012666e[0m      [37m0f[37m85[37m1b[37m01[32m00[32m00[0m   [32mjne 0x14012678f[0m[0m
[36m│[0m     [36m│[0m[36m└[0m[36m─[0m[36m─[0m[36m>[0m [32m0x140126674[0m      [33m48[37m8d[37m0d[37mbd[33m2e[33m58[37m.[0m  [37mlea[36m rcx[0m,[36m[36m [0m[[36m[33m0x1406a9538[0m][36m[0m[0m[31m     [31m; "main"[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x14012667b[0m      [37mba[37m05[32m00[32m00[32m00[0m     [37mmov[36m edx[0m,[36m[36m [33m5[0m[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126680[0m      [37me8[37mdb[33m4f[33m29[32m00[0m     [1;92mcall 0x1403bb660[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126685[0m      [37m0f[37mb6[37m05[37me4[37mfe[33m69[37m.[0m  [37mmovzx[36m eax[0m,[36m byte[36m [0m[[36m[33m0x1407c6570[0m][36m[0m[0m[31m [31m; [0x1407c6570:1]=0[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x14012668c[0m      [37mba[37m05[32m00[32m00[32m00[0m     [37mmov[36m edx[0m,[36m[36m [33m5[0m[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126691[0m      [33m31[37mc9[0m           [33mxor[36m ecx[0m,[36m[36m ecx[0m[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126693[0m      [37me8[33m78[33m3b[33m2a[32m00[0m     [1;92mcall 0x1403ca210[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126698[0m      [33m48[37m85[37mc0[0m         [33mtest[36m rax[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m┌[0m[36m─[0m[36m─[0m[36m<[0m [32m0x14012669b[0m      [37m0f[37m84[37mdd[32m00[32m00[32m00[0m   [32mje 0x14012677e[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266a1[0m      [37mc7[32m00[33m6d[33m61[33m69[33m6e[0m   [37mmov dword[36m [0m[[36mrax[0m][36m[0m,[36m[36m [33m0x6e69616d[0m[0m[31m [31m; 'main'
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [31m                                                           [31m; [0x6e69616d:4]=-1[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266a7[0m      [33m48[37mc7[33m45[37mb0[37m05[32m00[37m.[0m  [37mmov qword[36m [0m[[34mvar_50h[0m][36m[0m,[36m[36m [33m5[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266af[0m      [33m48[37m89[33m45[37mb8[0m       [37mmov qword[36m [0m[[34mvar_48h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266b3[0m      [33m48[37mc7[33m45[37mc0[37m04[32m00[37m.[0m  [37mmov qword[36m [0m[[34mvar_40h[0m][36m[0m,[36m[36m [33m4[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266bb[0m      [33m48[37m8d[33m4d[37mb0[0m       [37mlea[36m rcx[0m,[36m[36m [0m[[34mvar_50h[0m][36m[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266bf[0m      [37me8[37m9c[37md1[37m02[32m00[0m     [1;92mcall 0x140153860[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266c4[0m      [33m48[37m89[33m45[37mb8[0m       [37mmov qword[36m [0m[[34mvar_48h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266c8[0m      [33m48[37m89[33m55[37mc0[0m       [37mmov qword[36m [0m[[34mvar_40h[0m][36m[0m,[36m[36m rdx[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266cc[0m      [33m48[37mb9[32m00[32m00[32m00[32m00[37m.[0m  [37mmovabs[36m rcx[0m,[36m[36m [33m0x8000000000000000[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266d6[0m      [33m48[37m89[33m4d[37mb0[0m       [37mmov qword[36m [0m[[34mvar_50h[0m][36m[0m,[36m[36m rcx[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266da[0m      [33m48[37m89[37mc1[0m         [37mmov[36m rcx[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266dd[0m      [37me8[37m0e[33m50[33m29[32m00[0m     [1;92mcall 0x1403bb6f0[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266e2[0m      [33m48[37m89[37mc1[0m         [37mmov[36m rcx[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266e5[0m      [37me8[33m46[33m51[33m29[32m00[0m     [1;92mcall 0x1403bb830[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266ea[0m      [33m48[37m8d[37m0d[33m2f[37mc1[31mff[37m.[0m  [37mlea[36m rcx[0m,[36m[36m [0m[[36m[33m0x140122820[0m][36m[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266f1[0m      [37me8[37m8a[37m14[37mee[31mff[0m     [1;92mcall 0x140007b80[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266f6[0m      [33m31[37mc0[0m           [33mxor[36m eax[0m,[36m[36m eax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266f8[0m      [33m48[37m8b[37m0d[37md9[37mfc[33m69[37m.[0m  [37mmov[36m rcx[0m,[36m qword[36m [0m[[36m[33m0x1407c63d8[0m][36m[0m[0m[31m [31m; [0x1407c63d8:8]=0[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x1401266ff[0m      [33m48[37m83[37mf9[37m03[0m       [33mcmp[36m rcx[0m,[36m[36m [33m3[0m[0m[31m                 [31m; 3[0m
[36m│[0m    [36m┌[0m[36m─[0m[36m─[0m[36m─[0m[36m─[0m[36m<[0m [32m0x140126703[0m      [33m74[33m39[0m           [32mje 0x14012673e[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126705[0m      [33m48[37m89[33m45[37me8[0m       [37mmov qword[36m [0m[[34mvar_18h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126709[0m      [37mc6[33m45[37mf7[37m01[0m       [37mmov byte[36m [0m[[34mvar_9h[0m][36m[0m,[36m[36m [33m1[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x14012670d[0m      [33m48[37m8d[33m45[37mf7[0m       [37mlea[36m rax[0m,[36m[36m [0m[[34mvar_9h[0m][36m[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126711[0m      [33m48[37m89[33m45[37ma8[0m       [37mmov qword[36m [0m[[34mvar_58h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126715[0m      [33m48[37m8d[37m05[37mf4[37m09[33m58[37m.[0m  [37mlea[36m rax[0m,[36m[36m [0m[[36m[33m0x1406a7110[0m][36m[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x14012671c[0m      [33m48[37m89[33m44[33m24[33m20[0m     [37mmov qword[36m [0m[[34mvar_20h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126721[0m      [33m48[37m8d[37m0d[37mb0[37mfc[33m69[37m.[0m  [37mlea[36m rcx[0m,[36m[36m [0m[[36m[33m0x1407c63d8[0m][36m[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126728[0m      [33m4c[37m8d[37m0d[37md9[37m15[33m58[37m.[0m  [37mlea[36m r9[0m,[36m[36m [0m[[36m[33m0x1406a7d08[0m][36m[0m[0m[31m      [31m; " O\x02@\x01"[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x14012672f[0m      [33m4c[37m8d[33m45[37ma8[0m       [37mlea[36m r8[0m,[36m[36m [0m[[34mvar_58h[0m][36m[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126733[0m      [33m31[37md2[0m           [33mxor[36m edx[0m,[36m[36m edx[0m[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x140126735[0m      [37me8[37m16[33m6c[33m48[32m00[0m     [1;92mcall 0x1405ad350[0m[0m
[36m│[0m    [36m│[0m[36m│[0m[36m│[0m[36m│[0m   [32m0x14012673a[0m      [33m48[37m8b[33m45[37me8[0m       [37mmov[36m rax[0m,[36m qword[36m [0m[[34mvar_18h[0m][36m[0m[0m[0m
[36m│[0m    [36m└[0m[36m─[0m[36m─[0m[36m─[0m[36m─[0m[36m>[0m [32m0x14012673e[0m      [33m48[37m81[37mc4[37ma0[32m00[32m00[37m.[0m  [33madd[36m rsp[0m,[36m[36m [33m0xa0[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x140126745[0m      [33m5d[0m             [35mpop[36m rbp[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m│[0m   [32m0x140126746[0m      [37mc3[0m             [31mret[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m└[0m[36m─[0m[36m>[0m [32m0x140126747[0m      [33m48[37m8d[37m05[37m8a[33m30[33m58[37m.[0m  [37mlea[36m rax[0m,[36m[36m [0m[[36m[33m0x1406a97d8[0m][36m[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x14012674e[0m      [33m48[37m89[33m45[37mb0[0m       [37mmov qword[36m [0m[[34mvar_50h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126752[0m      [33m48[37mc7[33m45[37mb8[37m01[32m00[37m.[0m  [37mmov qword[36m [0m[[34mvar_48h[0m][36m[0m,[36m[36m [33m1[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x14012675a[0m      [33m48[37m8d[37m05[37me7[37m02[33m5b[37m.[0m  [37mlea rax, str.All_types__._.CoCreateInstance___IFileOpenDialogIFileOpenDialog::GetResultsIShellItemArray::GetCountIShellItemArray::GetItemAtIShellItem::GetAttributesSHCreateItemFromParsingNameIModalWindow::ShowIFileDialog::SetFolderIFileDialog::SetFileNameIFileDialog::SetFileTypeIndexIFileDialog::GetOptionsIFileDialog::SetOptionsIFileDialog::SetTitleIFileDialog::SetFileTypesIFileDialog::GetFileTypeIndexIShellItem::GetDisplayName[0m[31m [31m; 0x1406d6a48[31m [31m; "All types (*.*)*.*CoCreateInstance - IFileOpenDialogIFileOpenDialog::GetResultsIShellItemArray::GetCountIShellItemArray::GetItemAtIShellItem::GetAttributesSHCreateItemFromParsingNameIModalWindow::ShowIFileDialog::SetFolderIFileDialog::SetFileNameIFileDialog::SetFileTypeIndexIFileDialog::GetOptionsIFileDialog::SetOptionsIFileDialog::SetTitleIFileDialog::SetFileTypesIFileDialog::GetFileTypeIndexIShellItem::GetDisplayName"[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126761[0m      [33m48[37m89[33m45[37mc0[0m       [37mmov qword[36m [0m[[34mvar_40h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126765[0m      [37m0f[33m57[37mc0[0m         [33mxorps[36m xmm0[0m,[36m[36m xmm0[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126768[0m      [37m0f[37m11[33m45[37mc8[0m       [37mmovups xmmword[36m [0m[[34mvar_38h[0m][36m[0m,[36m[36m xmm0[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x14012676c[0m      [33m48[37m8d[37m15[33m75[33m30[33m58[37m.[0m  [37mlea[36m rdx[0m,[36m[36m [0m[[36m[33m0x1406a97e8[0m][36m[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126773[0m      [33m48[37m8d[33m4d[37mb0[0m       [37mlea[36m rcx[0m,[36m[36m [0m[[34mvar_50h[0m][36m[0m[0m[0m
[36m│[0m     [36m│[0m[36m│[0m    [32m0x140126777[0m      [37me8[37m84[37m87[33m47[32m00[0m     [1;92mcall 0x14059ef00[0m[0m
[36m│[0m     [36m│[0m[36m│[0m[36m┌[0m[36m─[0m[36m<[0m [32m0x14012677c[0m      [37meb[33m46[0m           [32mjmp 0x1401267c4[0m[0m
[36m│[0m     [36m│[0m[36m└[0m[36m─[0m[36m─[0m[36m>[0m [32m0x14012677e[0m      [37mb9[37m01[32m00[32m00[32m00[0m     [37mmov[36m ecx[0m,[36m[36m [33m1[0m[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126783[0m      [37mba[37m05[32m00[32m00[32m00[0m     [37mmov[36m edx[0m,[36m[36m [33m5[0m[0m[0m
[36m│[0m     [36m│[0m [36m│[0m   [32m0x140126788[0m      [37me8[37m93[37m83[33m47[32m00[0m     [1;92mcall 0x14059eb20[0m[0m
[36m│[0m     [36m│[0m[36m┌[0m[36m─[0m[36m─[0m[36m<[0m [32m0x14012678d[0m      [37meb[33m35[0m           [32mjmp 0x1401267c4[0m[0m
[36m│[0m     [36m└[0m[36m─[0m[36m─[0m[36m─[0m[36m>[0m [32m0x14012678f[0m      [33m48[37m8d[37m05[33m72[33m2f[33m58[37m.[0m  [37mlea[36m rax[0m,[36m[36m [0m[[36m[33m0x1406a9708[0m][36m[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x140126796[0m      [33m48[37m89[33m45[37mb0[0m       [37mmov qword[36m [0m[[34mvar_50h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x14012679a[0m      [33m48[37mc7[33m45[37mb8[37m01[32m00[37m.[0m  [37mmov qword[36m [0m[[34mvar_48h[0m][36m[0m,[36m[36m [33m1[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267a2[0m      [33m48[37m8d[37m05[37m9f[37m02[33m5b[37m.[0m  [37mlea rax, str.All_types__._.CoCreateInstance___IFileOpenDialogIFileOpenDialog::GetResultsIShellItemArray::GetCountIShellItemArray::GetItemAtIShellItem::GetAttributesSHCreateItemFromParsingNameIModalWindow::ShowIFileDialog::SetFolderIFileDialog::SetFileNameIFileDialog::SetFileTypeIndexIFileDialog::GetOptionsIFileDialog::SetOptionsIFileDialog::SetTitleIFileDialog::SetFileTypesIFileDialog::GetFileTypeIndexIShellItem::GetDisplayName[0m[31m [31m; 0x1406d6a48[31m [31m; "All types (*.*)*.*CoCreateInstance - IFileOpenDialogIFileOpenDialog::GetResultsIShellItemArray::GetCountIShellItemArray::GetItemAtIShellItem::GetAttributesSHCreateItemFromParsingNameIModalWindow::ShowIFileDialog::SetFolderIFileDialog::SetFileNameIFileDialog::SetFileTypeIndexIFileDialog::GetOptionsIFileDialog::SetOptionsIFileDialog::SetTitleIFileDialog::SetFileTypesIFileDialog::GetFileTypeIndexIShellItem::GetDisplayName"[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267a9[0m      [33m48[37m89[33m45[37mc0[0m       [37mmov qword[36m [0m[[34mvar_40h[0m][36m[0m,[36m[36m rax[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267ad[0m      [37m0f[33m57[37mc0[0m         [33mxorps[36m xmm0[0m,[36m[36m xmm0[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267b0[0m      [37m0f[37m11[33m45[37mc8[0m       [37mmovups xmmword[36m [0m[[34mvar_38h[0m][36m[0m,[36m[36m xmm0[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267b4[0m      [33m48[37m8d[37m15[37m95[33m2f[33m58[37m.[0m  [37mlea[36m rdx[0m,[36m[36m [0m[[36m[33m0x1406a9750[0m][36m[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267bb[0m      [33m48[37m8d[33m4d[37mb0[0m       [37mlea[36m rcx[0m,[36m[36m [0m[[34mvar_50h[0m][36m[0m[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [32m0x1401267bf[0m      [37me8[33m3c[37m87[33m47[32m00[0m     [1;92mcall 0x14059ef00[0m[0m
[36m│[0m      [36m│[0m[36m│[0m   [31m[31m; CODE XREFS from main @ [31m0x14012677c[31m, 0x14012678d[31m[0m
[36m└[0m      [36m└[0m[36m└[0m[36m─[0m[36m>[0m [32m0x1401267c4[0m      [37m0f[37m0b[0m           [1;91mud2[0m[0m[0m
