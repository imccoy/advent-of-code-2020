#include <stdlib.h>
#include <stdio.h>

int main(void) 
{
	int nums_map[2021] = {0};
	unsigned int nums_sorted[300] = {0};
	unsigned int size = 0;
	unsigned int current_n;

	while (scanf("%u", &current_n) == 1) {
		nums_map[current_n] = 1;
	}
	for (int i = 0; i < sizeof(nums_map) / sizeof(int); i++) {
		if (nums_map[i]) {
			nums_sorted[size++] = i;
		}
	}

	for (int k = size - 1; k > 2; k--) {
		int nk = nums_sorted[k];
		for (int j = 0; j < k - 2; j++) {
			int nj = nums_sorted[j];
			if (nj + nk >= 2020)
				break;
			int ni = 2020 - nj - nk;
			if (nums_map[ni] != 0) {
				printf("%d\n", ni * nj * nk);
				exit(0);
			}
		}
	}
}
