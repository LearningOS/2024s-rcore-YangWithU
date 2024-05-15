    .section .text.entry # 将之后内容全放到.text.entry, .entry保证优先执行
    .globl _start # 开一个全局符号 _start
_start:
    li x1, 100 # li立即数 100 存到x1寄存器