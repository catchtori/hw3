<center>

# Lab 6b: LC-3 Executor

Author : 栗威
Major : 计算机科学与技术
ID : 3210106175
</center>

---

## 1.Introduction 
Write a program to execute LC-3 binary code. I choose C to implement the program.

We need to implement 16 instructions in theory, their 4-bit opcodes are from 0000 to 1111. However, 1000 RTI and 1101 are not required. And in the 1111 TRAP instruction, we only need to implement the HALT. When HALT instruction is executed, the executor should stop and quit.

## 2.Algorithm

### 2.1 Data Structure
To implement the instructions and execute them successfully, we need to create a virtual machine, which stores PC, memory locations, the state of 8 registers and condition codes nzp. In fact, all the following instructions are just change the variables in virtual machine.
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
![avatar](https://github.com/catchtori/hw3/blob/master/%E6%9C%AA%E5%91%BD%E5%90%8D%E6%96%87%E4%BB%B6.png?raw=true)

The main function just initialize the virtual machine, and run it:
```c
int main(){
    Init(&vm);
    Run(&vm);
    return 0;
}
```
In Init(&vm), it initialize the variables of the virtual machine, read in the instructions from stdin and store them into ROM. 
The initialization contains: setting all the memory and register location to x0777; setting condition code nzp to 0; reading the start address and set PC at start address; then reading in instructions one by one and putting them behind PC.
```c
void Init(VM* vm){
    int t = 0;
    char temp[17] = "0111011101110111"; //x0777
    for(t=0; t<65536; t++){
        memcpy(vm->rom[t], temp, 16);
    }
    vm->n = 0;  vm->z = 0;  vm->p = 0;
    vm->R[0] = 30583; vm->R[1] = 30583; vm->R[2] = 30583; vm->R[3] = 30583; 
    vm->R[4] = 30583; vm->R[5] = 30583; vm->R[6] = 30583; vm->R[7] = 30583;
    t = 0;
    char c = '\0';
    char *address = malloc(sizeof(INST) + 1);
    while (t!=16){
        c=getchar();
        address[t++] = c;
    }
    address[16] = '\0';
    vm->pc = (uint16_t)cton(address, 16);
    free(address);
    t = 0;
    while ((c=getchar())!= EOF){
        if(c!=10){
            vm->rom[vm->pc][t++] = c;
        }
    }

}
```
In Run(&vm), it read the instructions from ROM one by one,then increment PC and match the cases to execute right intruction.
```c
INST *inst = malloc(sizeof(INST) + 1);
    ((char*)inst)[16] = '\0';
    while(1){
    // fetch the opcode
        memcpy(inst, m->rom[m->pc], sizeof(INST));
        m->pc += 1;
    //matching the instructions omitted
    }

```
### 2.3 Details

---

## 3. Test Result

## 4. Appendix Code
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>


// This function converts a binary string to an integer
int cton(char* str, int t) {
    int temp = 0; // Initialize a temporary variable to hold the converted integer
    for (int i = 0; i < t; i++) {
        // Left shift the temp variable by 1 and then bitwise OR it with the current binary digit
        // The current binary digit is obtained by subtracting '0' from the character at index i in the string
        temp = (temp << 1) | (str[i] - '0');
    }
    return temp; // Return the converted integer
}



// This function converts a binary string to an integer.
// Parameters:
// - str: The binary string to be converted.
// - t: The number of bits to consider in the conversion.
int ctou(const char* str, int t) {
    int rst = 0;             // Initialize the result variable to 0.
    int max = 1<<t;          // Calculate the maximum value that can be represented with t bits.
    int IsNeg = (*str == '1');  // Check if the first character of the string is '1' to determine if the number is negative.

    // Loop through each bit in the binary string.
    while (t--){
        rst = rst * 2 + (*str - '0');
        str++;
    }
    // If the number is negative, subtract the maximum value that can be represented with t bits from the result.
    if (IsNeg) {
        rst -= max;
    }

    // Return the final result.
    return rst;
}


// Function to convert an unsigned 16-bit integer to a string representation in a given radix
char* Itoa(uint16_t num, char* str, int radix) {
    // Array containing the characters representing the digits in the given radix
    const char index[] = {'0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'};
    
    // Variables for iterating through the string and array, and storing temporary values
    int i = 15, j, k = 15;
    char temp;
    
    // Convert the number to a string representation
    do {
        // Get the remainder of the number when divided by the radix and use it as an index to get the corresponding character
        str[i--] = index[num % radix];
        // Divide the number by the radix to get the next digit
        num /= radix;
        
    // Repeat until the number becomes zero (i.e., all digits have been processed)
    } while (num);

    // Reverse the string representation
    for (j = k; j <= (i - 1) / 2; j--) {
        // Swap characters from the start and end of the string
        temp = str[j];
        str[j] = str[i - 1 + k - j];
        str[i - 1 + k - j] = temp;
    }

    // Return the pointer to the string representation
    return str;
}

typedef struct{
    char opcode[4];
    char value[12];
} INST;

typedef struct{
    uint16_t pc;
    int n;
    int z;
    int p;
    char rom[65536][16];
    uint16_t R[8];
} VM;

VM vm;


void setCondition(VM *m, uint16_t value){
    // Check if the value is equal to 0
    if(value == 0){
        // If true, set the n flag to 0, p flag to 0, and z flag to 1
        m->n = 0;
        m->p = 0;
        m->z = 1;
    }
    // If the value is not equal to 0, check if the highest bit (bit 15) is set
    else if(value & (1<<15)){
        // If true, set the n flag to 1, p flag to 0, and z flag to 0
        m->n = 1;
        m->p = 0;
        m->z = 0;
    }
    // If the value is not equal to 0 and the highest bit is not set
    else{
        // Set the n flag to 0, p flag to 1, and z flag to 0
        m->n = 0;
        m->p = 1;
        m->z = 0;
    }
}

// Branch Instruction - BR
void BR(VM *m, INST* inst){
    // Check condition codes and perform branch if condition is met
    if((inst->value[0]-'0' && m->n) || (inst->value[1]-'0' && m->z) || (inst->value[2]-'0' && m->p)){
        // Extract and sign-extend the offset
        int SEXT = ctou((char*)inst->value + 3, 9);
        // Update the program counter (pc)
        m->pc += SEXT;
    }
}

// Add Instruction - ADD
void ADD(VM *m, INST* inst){
    // Extract destination register (DR) and source register 1 (SR1)
    int DR = cton((char*)inst->value, 3);
    int SR1 = cton((char*)inst->value + 3, 3);
    // Check if immediate mode is enabled
    if(inst->value[6] == '1'){
        // Extract and sign-extend the immediate value
        int SEXT = ctou((char*)inst->value + 7, 5);
        // Perform addition with immediate value and update destination register
        m->R[DR] = m->R[SR1] + SEXT;
    }else{
        // Extract source register 2 (SR2)
        int SR2 = cton((char*)inst->value + 9, 3);
        // Perform addition with source registers and update destination register
        m->R[DR] = m->R[SR1] + m->R[SR2];
    }
    // Update condition codes
    setCondition(m, m->R[DR]);
}

// Load Instruction - LD
void LD(VM *m, INST* inst){
    // Extract destination register (DR) and PCoffset9
    int DR = cton((char*)inst->value, 3);
    int PCoffset9 = ctou((char*)inst->value + 3, 9);
    // Load the value from memory into the destination register
    m->R[DR] = cton(m->rom[m->pc + PCoffset9], 16);
    // Update condition codes
    setCondition(m, m->R[DR]);
}

// Store Instruction - ST
void ST(VM *m, INST* inst){
    // Extract source register (SR) and PCoffset9
    int SR = cton((char*)inst->value, 3);
    int PCoffset9 = ctou((char*)inst->value + 3, 9);
    // Create temporary buffer to store the binary representation of source register value
    char * temp = malloc(sizeof(INST) + 1);
    memset(temp, '0', 16);
    // Convert source register value to binary and store it in memory
    memcpy(m->rom[m->pc + PCoffset9], Itoa(m->R[SR], temp, 2), 16);
    free(temp);
}

// This function performs a JSR (Jump to Subroutine) instruction.
// It takes a VM (Virtual Machine) object and an INST (Instruction) object as parameters.
void JSR(VM *m, INST* inst){
    // Save the current program counter in a temporary variable.
    uint16_t temp = m->pc;
    
    // Check if the first bit of the instruction value is '1'.
    if(inst->value[0] == '1'){
        // If it is, extract the 11-bit PCoffset11 value from the instruction value.
        int PCoffset11 = ctou((char*)inst->value + 1, 11);
        
        // Update the program counter by adding the PCoffset11 value.
        m->pc = m->pc + PCoffset11;
    }else{
        // If the first bit of the instruction value is not '1',
        // extract the 3-bit BaseR value from the instruction value.
        int BaseR = cton((char*)inst->value + 3, 3);
        
        // Update the program counter by setting it to the value of the BaseR register.
        m->pc = m->R[BaseR];
    }
    
    // Save the original program counter value in register R7.
    m->R[7] = temp;
}

// This function performs an AND instruction.
// It takes a VM (Virtual Machine) object and an INST (Instruction) object as parameters.
void AND(VM *m, INST* inst){
    // Extract the 3-bit DR (Destination Register) value from the instruction value.
    int DR = cton((char*)inst->value, 3);
    
    // Extract the 3-bit SR1 (Source Register 1) value from the instruction value.
    int SR1 = cton((char*)inst->value + 3, 3);
    
    // Check if the sixth bit of the instruction value is '1'.
    if(inst->value[6] == '1'){
        // If it is, extract the 5-bit SEXT (Sign-Extended Immediate) value from the instruction value.
        uint16_t SEXT = (uint16_t)ctou((char*)inst->value + 7, 5);
        
        // Perform the bitwise AND operation between the value in SR1 register and SEXT,
        // and store the result in the DR register.
        m->R[DR] = m->R[SR1] & SEXT;
    }else{
        // If the sixth bit of the instruction value is not '1',
        // extract the 3-bit SR2 (Source Register 2) value from the instruction value.
        int SR2 = cton((char*)inst->value + 9, 3);
        
        // Perform the bitwise AND operation between the values in SR1 and SR2 registers,
        // and store the result in the DR register.
        m->R[DR] = m->R[SR1] & m->R[SR2];
    }
    
    // Update the condition flags based on the value in the DR register.
    setCondition(m, m->R[DR]);
}
void LDR(VM *m, INST* inst){
    // Extract the destination register (DR), base register (BaseR), and offset6 from the instruction value
    int DR = cton((char*)inst->value, 3);
    int BaseR = cton((char*)inst->value + 3, 3);
    int offset6 = ctou((char*)inst->value + 6, 6);

    // Load the value from memory at the address calculated by adding the base register and offset6
    // into the destination register (DR)
    m->R[DR] = cton(m->rom[m->R[BaseR] + offset6], 16);

    // Set the condition flags based on the value stored in the destination register (DR)
    setCondition(m, m->R[DR]);      
}

void STR(VM *m, INST* inst){
    // Extract the source register (SR), base register (BaseR), and offset6 from the instruction value
    int SR = cton((char*)inst->value, 3);
    int BaseR = cton((char*)inst->value + 3, 3);
    int offset6 = ctou((char*)inst->value + 6, 6);

    // Create a temporary buffer to store the binary representation of the source register (SR) value
    char * temp = malloc(sizeof(INST) + 1);
    memset(temp, '0', 16);

    // Convert the source register (SR) value to binary and copy it to the memory location calculated
    // by adding the base register and offset6
    memcpy(m->rom[m->R[BaseR] + offset6], Itoa(m->R[SR], temp, 2), 16);

    // Free the temporary buffer
    free(temp); 
}
void NOT(VM *m, INST* inst){
    // Extract the destination register (DR) and source register (SR) from the instruction value
    int DR = cton((char*)inst->value, 3);
    int SR = cton((char*)inst->value + 3, 3);

    // Perform bitwise NOT operation on the value in source register (SR)
    m->R[DR] = ~m->R[SR];

    // Update the condition flags based on the result in destination register (DR)
    setCondition(m, m->R[DR]);    
}

void LDI(VM *m, INST* inst){
    // Extract the destination register (DR) from the instruction value
    int DR = cton((char*)inst->value, 3);

    // Extract the PCoffset9 from the instruction value
    int PCoffset9 = ctou((char*)inst->value + 3, 9);

    // Calculate the address by adding the PCoffset9 to the program counter (pc)
    int address = cton(m->rom[m->pc + PCoffset9], 16);

    // Load the value from the memory at the calculated address into the destination register (DR)
    m->R[DR] = cton(m->rom[address], 16);

    // Update the condition flags based on the value loaded into the destination register (DR)
    setCondition(m, m->R[DR]);    
}
void STI(VM *m, INST* inst){
    // Extract the SR field from the instruction value
    int SR = cton((char*)inst->value, 3);
    
    // Extract the PCoffset9 field from the instruction value
    int PCoffset9 = ctou((char*)inst->value + 3, 9);
    
    // Allocate memory for a temporary string
    char* temp = malloc(sizeof(INST) + 1);
    
    // Calculate the target address by adding the PCoffset9 to the current program counter
    int address = cton(m->rom[m->pc + PCoffset9], 16);
    
    // Initialize the temporary string with '0' characters
    memset(temp, '0', 16);
    
    // Convert the value in register SR to a binary string and copy it to the target address
    memcpy(m->rom[address], Itoa(m->R[SR], temp, 2), 16);
    
    // Free the memory allocated for the temporary string
    free(temp);
}

void JMP(VM *m, INST* inst){
    // Extract the BaseR field from the instruction value
    int BaseR = cton((char*)inst->value + 3, 3);
    
    // Set the program counter to the value in register BaseR
    m->pc = m->R[BaseR];
}

void LEA(VM *m, INST* inst){
    // Extract the DR field from the instruction value
    int DR = cton((char*)inst->value, 3);
    
    // Extract the PCoffset9 field from the instruction value
    int PCoffset9 = ctou((char*)inst->value + 3, 9);
    
    // Calculate the effective address by adding the PCoffset9 to the current program counter
    m->R[DR] = m->pc + PCoffset9;
}

void OUT(VM *m){
    int t = 0;
    for(t = 0; t<=7 ; t++) printf("R%d = x%04hX\n", t, m->R[t]);

#ifdef DEBUG
    printf("n = %d\n", m->n);
    printf("z = %d\n", m->z);
    printf("p = %d\n", m->p);
#endif
}

// Run to execute the program
int Run(VM *m) {
    // Allocate memory for the instruction
    INST *inst = malloc(sizeof(INST) + 1);
    ((char*)inst)[16] = '\0'; // Null-terminate the instruction

    while (1) {
        // Fetch the opcode from the ROM
        memcpy(inst, m->rom[m->pc], sizeof(INST));

#ifdef DEBUG
        // Print opcode and value for debugging
        printf("[DEBUG] opcode: %0.4s, value: %0.12s\n", inst->opcode, inst->value);
        printf("[DEBUG] pc: %d\n", m->pc);
#endif

        // Increment the program counter (pc)
        m->pc += 1;

        // Execute the instruction based on the opcode
        switch (cton(inst->opcode, 4)) {
            case 0: {
                // Branch instruction
                BR(m, inst);
                break;
            }
            case 1: {
                // Add instruction
                ADD(m, inst);
                break;
            }
            case 2: {
                // Load instruction
                LD(m, inst);
                break;
            }
            case 3: {
                // Store instruction
                ST(m, inst);
                break;
            }
            case 4: {
                // Jump subroutine instruction
                JSR(m, inst);
                break;
            }
            case 5: {
                // Bitwise AND instruction
                AND(m, inst);
                break;
            }
            case 6: {
                // Load register instruction
                LDR(m, inst);
                break;
            }
            case 7: {
                // Store register instruction
                STR(m, inst);
                break;
            }
            // RTI is not required
            case 9: {
                // Bitwise NOT instruction
                NOT(m, inst);
                break;
            }
            case 10: {
                // Load indirect instruction
                LDI(m, inst);
                break;
            }
            case 11: {
                // Store indirect instruction
                STI(m, inst);
                break;
            }
            case 12: {
                // Jump instruction
                JMP(m, inst);
                break;
            }
            // 1101 is not required
            case 14: {
                // Load effective address instruction
                LEA(m, inst);
                break;
            }
            case 15: {
                // Output instruction
                if (cton(inst->value, 12) == 37) {
                    OUT(m);
                    return 0;
                }
                break;
            }
            default:
                printf("Error!\n");
                exit(-1);
        }
    }
}
//Initialization
void Init(VM* vm){
    // Initialize variables
    int t = 0;
    char temp[17] = "0111011101110111"; // set x0777

    // Set the ROM
    for(t=0; t<65536; t++){
        memcpy(vm->rom[t], temp, 16);
    }

    // Initialize registers
    vm->n = 0;  vm->z = 0;  vm->p = 0;
    vm->R[0] = 30583; vm->R[1] = 30583; vm->R[2] = 30583; vm->R[3] = 30583; 
    vm->R[4] = 30583; vm->R[5] = 30583; vm->R[6] = 30583; vm->R[7] = 30583;
    
    // Read the starting address from user input
    t = 0;
    char c = '\0';
    char *address = malloc(sizeof(INST) + 1);
    while (t!=16){
        c=getchar();
        address[t++] = c;
    }
    address[16] = '\0';
    vm->pc = (uint16_t)cton(address, 16);
    free(address);
    
    // Read the program instructions from user input
    t = 0;
    while ((c=getchar())!= EOF){
        if(c!=10){
            vm->rom[vm->pc][t++] = c;
        }
    }

#ifdef DEBUG
    printf("ok!\n");
    printf("starting address = %d\n", vm->pc);
#endif
}

int main(){
    // Initialize the vm
    Init(&vm);
    
    // Run the vm
    Run(&vm);
    
    // Return 0 to indicate successful execution
    return 0;
}
//😀
```