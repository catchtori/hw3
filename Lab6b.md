<center>
# Lab 6b: LC-3 Executor
<font size = 5>

Author : 栗威
<br>
Major : 计算机科学与技术
<br>
ID : 3210106175
</center>
</font>
<font size = 4>
___

## 1.Introduction ##



Write a program to execute LC-3 binary code. I choose C to implement the program.
<br><br>
We need to implement 16 instructions in theory, their 4-bit opcodes are from 0000 to 1111. However, 1000 RTI and 1101 are not required. And in the 1111 TRAP instruction, we only need to implement the HALT. When HALT instruction is executed, the executor should stop and quit.
---
## 2.Algorithm

### 2.1 Data Structure
To implement the instructions and execute them successfully, we need to create a virtual machine, which stores PC, memory locations, the state of 8 registers and condition codes nzp. In fact, all the following instructions are just change the variables in virtual machine.<br>
So I define a struct to represent the virtual machine:
```c
typedef struct{
    uint16_t pc;
    int n;
    int z;
    int p;
    char rom[65536][16];
    uint16_t R[8];
} VM;
```

Also, to help handle with instructions, I define a struct which divide a instruction into opcode and value 2 parts:
```c
typedef struct{
    char opcode[4];
    char value[12];
} INST;
```



### 2.2 Program Process
![avatar](https://github.com/catchtori/hw3/blob/master/%E6%9C%AA%E5%91%BD%E5%90%8D%E6%96%87%E4%BB%B6.png?raw=true）

### 2.3 Details

---

## 3. Test Result

## 4. Appendix Code