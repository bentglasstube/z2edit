#ifndef Z2UTIL_IMWIDGET_MISC_HACKS_H
#define Z2UTIL_IMWIDGET_MISC_HACKS_H
#include "imwidget/imwidget.h"

class Mapper;
namespace z2util {

class MiscellaneousHacks: public ImWindowBase {
  public:
    MiscellaneousHacks(): ImWindowBase(false) {};

    bool Draw() override;
    inline void set_mapper(Mapper* m) { mapper_ = m; };
  private:
    Mapper* mapper_;
};

}  // namespace
#endif // Z2UTIL_IMWIDGET_MISC_HACKS_H
