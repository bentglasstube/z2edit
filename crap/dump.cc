#include <gflags/gflags.h>

#include "overworld.h"
#include "romfile.h"

#include "util/strutil.h"

DEFINE_string(romfile, "", "NES rom to inspect.");
DEFINE_string(hexdump, "", "Hexdump offset:length or bank:addr:length");
DEFINE_int32(ovrstart, 0, "Overworld start.");
DEFINE_int32(ovrend, 0, "Overworld end.");
DEFINE_int32(ovrareas, 0, "Overworld areas pointer.");
DEFINE_string(grep, "", "Search for binary pattern and hexdump it");

int main(int argc, char *argv[]) {
    gflags::ParseCommandLineFlags(&argc, &argv, true);

    if (FLAGS_romfile.empty()) {
        fprintf(stderr, "Must specify a romfile!\n");
        exit(1);
    }

    RomFile rom;
    rom.Load(FLAGS_romfile);

    if (!FLAGS_hexdump.empty()) {
        auto h = Split(FLAGS_hexdump, ":");
        uint32_t bank, addr, len;
        if (h.size() == 2) {
            addr = strtoul(h[0].c_str(), 0, 0);
            len = strtoul(h[1].c_str(), 0, 0);
            rom.Hexdump(addr, len);
        } else if (h.size() == 3) {
            bank = strtoul(h[0].c_str(), 0, 0);
            addr = strtoul(h[1].c_str(), 0, 0);
            len = strtoul(h[2].c_str(), 0, 0);
            rom.Hexdump(bank, addr, len);
        } else {
            fprintf(stderr, "Unknown hexdump spec!\n");
            exit(1);
        }
    }
    if (!FLAGS_grep.empty()) {
        rom.Grep(FLAGS_grep);
    }

    if (FLAGS_ovrstart && FLAGS_ovrend) {
        Overworld over(&rom);
        over.Decompress(FLAGS_ovrstart, FLAGS_ovrend);
        over.Print();
    }
    if (FLAGS_ovrareas) {
        Overworld over(&rom);
        over.PrintAreas(FLAGS_ovrareas);
    }

    return 0;
}
