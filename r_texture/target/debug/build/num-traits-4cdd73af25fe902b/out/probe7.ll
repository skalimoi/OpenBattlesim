; ModuleID = 'probe7.e97035a4f649e06e-cgu.0'
source_filename = "probe7.e97035a4f649e06e-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

; core::num::<impl u32>::to_ne_bytes
; Function Attrs: inlinehint uwtable
define internal i32 @"_ZN4core3num21_$LT$impl$u20$u32$GT$11to_ne_bytes17hbee4bc9fff0df0e7E"(i32 %self) unnamed_addr #0 {
start:
  %0 = alloca [4 x i8], align 1
  store i32 %self, ptr %0, align 1
  %1 = load i32, ptr %0, align 1
  ret i32 %1
}

; probe7::probe
; Function Attrs: uwtable
define void @_ZN6probe75probe17h976f1964c7fd361dE() unnamed_addr #1 {
start:
  %0 = alloca i32, align 4
  %_1 = alloca [4 x i8], align 1
; call core::num::<impl u32>::to_ne_bytes
  %1 = call i32 @"_ZN4core3num21_$LT$impl$u20$u32$GT$11to_ne_bytes17hbee4bc9fff0df0e7E"(i32 1)
  store i32 %1, ptr %0, align 4
  call void @llvm.memcpy.p0.p0.i64(ptr align 1 %_1, ptr align 4 %0, i64 4, i1 false)
  ret void
}

; Function Attrs: nocallback nofree nounwind willreturn memory(argmem: readwrite)
declare void @llvm.memcpy.p0.p0.i64(ptr noalias nocapture writeonly, ptr noalias nocapture readonly, i64, i1 immarg) #2

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nounwind willreturn memory(argmem: readwrite) }

!llvm.module.flags = !{!0}

!0 = !{i32 8, !"PIC Level", i32 2}
