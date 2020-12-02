#include <stdio.h>

int main(void) 
{
	unsigned char bits[253] = {0}; // 253 = ceil(2020/8)
	unsigned int current_n;
	unsigned int current_n_byte;
	unsigned int current_n_bit;

	unsigned int complement;
	unsigned int complement_byte;
	unsigned int complement_bit;
	
	while (scanf("%u", &current_n) == 1) {
		complement = 2020 - current_n;
		complement_byte = complement >> 3;
		complement_bit = complement & 7;
		if (bits[complement_byte] & (1 << complement_bit)) {
			printf("%d\n", complement * current_n);
		}

		current_n_byte = current_n >> 3;
		current_n_bit = current_n & 7;
		bits[current_n_byte] |= (1 << current_n_bit);
	}
}
