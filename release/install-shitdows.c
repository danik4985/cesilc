#include <stdio.h>
#include <stdlib.h>
#include "cesilc.h"

void pause() {
	printf("Any key to continue...");
	system("pause >nul");
}

int main(int argc, char const *argv[]) {
	#ifndef _WIN32
		printf("This is an installer for shitdows, please use install-linux.sh\n");
		return -1;
	#endif

	// We are running on shitdows
	system("color f0");
	system("title CesilC Installer");

	FILE* file;

	if ((file = fopen("C:\\Windows\\System32\\cesilc.exe", "wb+")) != NULL) {
		printf("Working, please wait...\n");
		fwrite(release_cesilc_exe, 1, release_cesilc_exe_len, file);
		fclose(file);
	} else {
		// An error has occurred

		if (argc > 1 && argv[1] == "att") {
			// Ok it's not a problem with the admin rights
			printf("An error has occurred, please try again later\n");
			pause();
			return -1;
		} else {
			char command[1024];
			snprintf(command, 1024, "runas %s att", argv[0]);
			return system(command);;
		}
		
	}

	printf("Done!\n");
	pause();

	return 0;
}

