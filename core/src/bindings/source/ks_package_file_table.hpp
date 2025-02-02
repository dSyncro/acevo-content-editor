#ifndef _KS_PACKAGE_FILE_TABLE_H
#define _KS_PACKAGE_FILE_TABLE_H

#include <stdint.h>

namespace ks {
    enum class PackageFileFlags : uint16_t
    {
        Directory = (1 << 0),
        XorCipher = (1 << 8),
    } ;

    struct PackageFileTableEntry
    {
        uint8_t          file_path[0xE0];
        int32_t          aligmt_E0;
        PackageFileFlags inf_flags;
        int16_t          path_leng;
        uint64_t         path_fnv1;
        uint64_t         file_size;
        uint64_t         file_offs;
    } ;

    const size_t PACKAGE_FILE_ENTRIES = 0x20000;
    const size_t PACKAGE_FILE_TABLE_SIZE = PACKAGE_FILE_ENTRIES * sizeof(PackageFileTableEntry);
}

#endif