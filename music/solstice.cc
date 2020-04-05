#include "music.h"

#include <iostream>

void dump_notes(std::vector<z2music::Note> notes) {
  for (z2music::Note n : notes) {
    std::cout << n.pitch_string();
    size_t left = n.length() / 4 - 4;
    std::cout << std::string(left, '.');
  }
  std::cout << std::endl;
}

void dump_song(const z2music::Song& song) {
  std::cout << "Song length: " << song.sequence_length() << " phrases" << std::endl;

  for (size_t i = 0; i < song.sequence_length(); ++i) {
    const z2music::Pattern* p = song.at(i);
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse1));
    dump_notes(p->notes(z2music::Pattern::Channel::Pulse2));
    dump_notes(p->notes(z2music::Pattern::Channel::Triangle));
    dump_notes(p->notes(z2music::Pattern::Channel::Noise));
    std::cout << std::endl;
  }
}

void set_final_boss(z2music::Song* boss) {
  boss->clear();

  boss->add_pattern({
      0x10,
      { 0xe6, 0xf4, 0xac, 0xea, 0xa6, 0xe6, 0xf4, 0xac, 0x2b, 0xe6, 0xf4, 0xac, 0xaa, 0xa6, 0xaa, 0x67, 0x65 },
      { 0x4f, 0x82, 0xce, 0x8e, 0x53, 0x82, 0xd2, 0x92, 0x55, 0x82, 0xd4, 0x94, 0x59, 0x59 },
      { 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0 },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88 },
      });

  boss->add_pattern({
      0x10,
      { 0xe6, 0xea, 0xec, 0xaa, 0xa6, 0x35, 0xec, 0xea, 0xa6, 0xe6, 0xea, 0xac, 0xea, 0xa6, 0x65, 0x43 },
      { 0x55, 0x82, 0xd4, 0x94, 0x53, 0x82, 0xd2, 0x92, 0x4f, 0x82, 0xce, 0x8e, 0x4f, 0x4f },
      { 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08 },
      });

  boss->add_pattern({
      0x10,
      { 0xe6, 0xf4, 0xac, 0xea, 0xa6, 0xe6, 0xea, 0xac, 0xaa, 0xc2, 0xe6, 0xf4, 0xac, 0xaa, 0xa6, 0xaa, 0x65, 0x43 },
      { 0x4f, 0x82, 0xce, 0x8e, 0x53, 0x82, 0xd2, 0x92, 0x55, 0x82, 0xd4, 0x94, 0x59, 0x59 },
      { 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0 },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88 },
      });

  boss->add_pattern({
      0x10,
      { 0xe6, 0xea, 0xac, 0xe6, 0xa6, 0xf4, 0xf6, 0xb4, 0xec, 0x82, 0xea, 0xec, 0xaa, 0xa6, 0xa4, 0xaa, 0x67, 0x65 },
      { 0x55, 0x82, 0xd4, 0x94, 0x53, 0x82, 0xd2, 0x92, 0x4f, 0x82, 0xce, 0x8e, 0x4f, 0x4f },
      { 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08 },
      });

  boss->add_pattern({
      0x10,
      { 0xa6, 0xaa, 0xac, 0xb0, 0x75, 0xa6, 0xaa, 0xac, 0xb0, 0xb4, 0xb6, 0xf4, 0xa6, 0xaa, 0xac, 0xb0, 0xb4, 0xb6, 0xb8, 0xb6, 0xb4, 0xb0, 0xac, 0xaa, 0xac, 0xaa, 0xa6, 0xa4 },
      { 0x4f, 0x82, 0xce, 0x8e, 0x53, 0x82, 0xd2, 0x92, 0x55, 0x82, 0xd4, 0x94, 0x59, 0x59 },
      { 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa4, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0, 0xa0 },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88 },
      });

  boss->add_pattern({
      0x10,
      { 0xa6, 0xaa, 0xac, 0xb0, 0xac, 0xaa, 0xe6, 0xf4, 0xb6, 0xb8, 0xb6, 0xb4, 0xf0, 0xaa, 0xa6, 0xa4, 0xa0, 0xa4, 0xa6, 0xaa, 0xac, 0xaa, 0xa6, 0xa4, 0xa6, 0xaa, 0xac, 0xe4 },
      { 0x55, 0x82, 0xd4, 0x94, 0x53, 0x82, 0xd2, 0x92, 0x4f, 0x82, 0xce, 0x8e, 0x4f, 0x4f },
      { 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0xa6, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c },
      { 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0xc8, 0xc8, 0xc8, 0x88, 0x88, 0x88, 0x88, 0x88, 0x88, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08, 0x08 },
      });

  boss->set_sequence({0, 1, 2, 3, 4, 5});
}

void set_gp_intro(z2music::Song* intro) {
  intro->clear();

  intro->add_pattern({
      0x18,
      { 0xa2, 0xe2, 0xe2, 0x23, 0x29, 0x27, 0xe4 },
      { 0x98, 0xd8, 0xd8, 0x19, 0x1f, 0x1d, 0xda },
      { 0xa2, 0xe2, 0xe2, 0x23, 0xa8, 0xa8, 0xa8, 0xa6, 0xa6, 0xa6, 0xa4, 0xa4 },
      {},
      });

  intro->set_sequence({0});
}

void set_gp_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x18,
      { 0x23, 0x19, 0xc2, 0xe2, 0x22, 0x26, 0x28, 0x2c, 0x71, 0xac, 0xa8, 0xec, 0xe8, 0xe6 },
      { 0x15, 0x0b, 0xc2, 0x59, 0x1f, 0x1b, 0x19, 0x15 },
      { 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0 },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });
  theme->add_pattern({
      0x18,
      { 0xa8, 0xa6, 0x63, 0x43, 0xc2, 0xac, 0xa8, 0x67, 0x43, 0xc2 },
      { 0x90, 0x8a, 0x59, 0x43, 0x82, 0x98, 0x94, 0x98, 0x5b, 0x43, 0xc2 },
      { 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9e, 0xa6, 0xac, 0x9e, 0xa6, 0xac, 0x82, 0xa6, 0xac, 0x9e, 0xa6, 0xac },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->add_pattern({
      0x18,
      { 0xa8, 0xa6, 0x63, 0x43, 0xc2, 0xac, 0xb0, 0x73, 0x43, 0x82, 0xb0 },
      { 0x90, 0x8a, 0x59, 0x43, 0x82, 0x98, 0x94, 0x9e, 0x5b, 0xc2, 0x02, 0x10, 0x14, 0x18, 0x1a, 0x18, 0x1a, 0x1e },
      { 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9e, 0xa6, 0xac, 0x9e, 0xa6, 0xec, 0xa6, 0xac, 0x9e, 0xa6, 0xac },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->add_pattern({
      0x18,
      { 0x39, 0x71, 0x43, 0xb0, 0x37, 0x71, 0x43, 0xb0 },
      { 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x9a, 0x94, 0x94, 0x94, 0x94, 0x98, 0x94 },
      { 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0x98 },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->add_pattern({
      0x18,
      { 0x2d, 0x71, 0x43, 0xb0, 0x2d, 0x29, 0x27, 0x29 },
      { 0x90, 0x90, 0x90, 0x90, 0x94, 0x90, 0x98, 0x98, 0x98, 0x98, 0x9a, 0x9e, 0xb8, 0xb8, 0xb8, 0xb8, 0xb8, 0xb8, 0xa6, 0xa6, 0xa6, 0xa6, 0xa8, 0xa6 },
      { 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9e, 0xa6, 0xac, 0x9e, 0xa6, 0xec, 0xa6, 0xac, 0x9e, 0xa6, 0xac },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->add_pattern({
      0x18,
      { 0xb8, 0x38, 0x38, 0xb8, 0xb6, 0xb8, 0xb0, 0xb2, 0x32, 0x32, 0xb2, 0xb2, 0xb0, 0xac, 0xb0, 0x30, 0x30, 0xb0, 0xb0, 0xac, 0xb0, 0xb2, 0xb0, 0xb2, 0xb6, 0xb8, 0xb6 },
      { 0xe2, 0xa2, 0x9e, 0xa2, 0x9e, 0xda, 0x9a, 0x98, 0x9a, 0x98, 0xd4, 0x94, 0x94, 0x98, 0x9a, 0x98, 0x9a, 0x9e, 0x9e, 0xa2, 0x9e },
      { 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0xa2, 0xa8, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0xb0, 0x9e, 0xa6, 0x98 },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->add_pattern({
      0x18,
      { 0xb8, 0x38, 0x38, 0xb8, 0xb6, 0xb8, 0xb0, 0xb2, 0x32, 0x32, 0xb2, 0xac, 0xb0, 0xb2, 0xb0, 0x30, 0x30, 0xb0, 0xb0, 0xac, 0xb6, 0x63, 0xc2 },
      { 0x23, 0xa2, 0x9e, 0x9a, 0x19, 0x98, 0x94, 0x90, 0x0f, 0x8e, 0x8a, 0x86, 0x0b, 0x0b },
      { 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9a, 0xa2, 0xac, 0x9e, 0xa6, 0xac, 0x9e, 0xa6, 0xec, 0xa6, 0xac, 0x9e, 0xa6, 0xac },
      { 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x88, 0x88, 0x09, 0x88, 0x08, 0x08, 0x08, 0x08 },
      });

  theme->set_sequence({0, 1, 0, 2, 3, 4, 3, 4, 0, 1, 0 , 2, 5, 6});
}

void set_zelda_theme(z2music::Song* zelda) {
  zelda->clear();

  zelda->add_pattern({
      0x28,
      { 0xb0, 0xa8, 0xa2, 0x9a, 0xb0, 0xa8, 0xa2, 0x9a, 0xb0, 0xa8, 0xa2, 0x9a, 0xb0, 0xa8, 0xa2, 0x9a, 0xa6, 0x9e, 0x98, 0x90, 0xa6, 0x9e, 0x98, 0x90, 0xa6, 0x9e, 0x98, 0x90, 0xa6, 0x9e, 0x98, 0x90 },
      { 0x67, 0x43, 0x43, 0xc2, 0xde, 0x63, 0x43, 0x43, 0x43 },
      { 0x5b, 0x5b, 0x5b, 0x5b, 0x69, 0x69, 0x69, 0x69 },
      { 0x49, 0xc8, 0x88, 0x88 },
      });

  zelda->set_sequence({0});
}

void set_credits_song(z2music::Song* credits) {
  credits->clear();

  credits->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xd0, 0x1f, 0x1b, 0xd8, 0x1b, 0x19, 0xd0},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xf0, 0xec, 0xb0, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  credits->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xd0, 0x15, 0x19, 0xd0, 0x15, 0x11, 0xce},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xec, 0xe6, 0xac, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  credits->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xde, 0x1b, 0x19, 0xd0, 0xd4, 0xd8, 0xda, 0xd8},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xf0, 0xec, 0xb0, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  credits->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xde, 0x1b, 0x19, 0xd0, 0xd4, 0xd0, 0xce, 0xd0},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xec, 0xe6, 0xac, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  credits->set_sequence({0, 1, 0, 1, 2, 3});
}

void set_alt_credits_theme(z2music::Song* credits) {
  credits->clear();

  credits->add_pattern({
      0x18,
      { 0x69, 0xc2, 0x5f, 0xc2, 0x63, 0x43, 0x82, 0xe6, 0xa8, 0x67, 0x43, 0x82, 0x1f, 0x23, 0x21, 0x23, 0x27 },
      { 0x11, 0x11, 0x15, 0x19, 0x55, 0x43, 0x43, 0x0f, 0x0f, 0x11, 0x15, 0x19, 0x15, 0x11, 0x0f },
      { 0x90, 0x98, 0x9e, 0xa8, 0x9e, 0x98, 0x90, 0x98, 0x9e, 0xa8, 0x9e, 0x98, 0x90, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x90, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x8e, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x8e, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x84, 0x88, 0x8e, 0x94, 0x8e, 0x88, 0x84, 0x88, 0x8e, 0x94, 0x8e, 0x88 },
      { 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0xc8, 0x88 },
      });

  credits->add_pattern({
      0x18,
      { 0x69, 0xc2, 0x5f, 0xc2, 0x5d, 0x43, 0x82, 0xde, 0xa2, 0x27, 0x27, 0x1d, 0x1d, 0x1f, 0x1d, 0x19, 0x1d },
      { 0x11, 0x11, 0x15, 0x19, 0x55, 0x43, 0x43, 0x1d, 0x1d, 0x15, 0x15, 0x19, 0x15, 0x11, 0x15 },
      { 0x90, 0x98, 0x9e, 0xa8, 0x9e, 0x98, 0x90, 0x98, 0x9e, 0xa8, 0x9e, 0x98, 0x90, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x90, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x8e, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x8e, 0x94, 0x9c, 0xa2, 0x9c, 0x94, 0x84, 0x88, 0x8e, 0x94, 0x8e, 0x88, 0x84, 0x88, 0x8e, 0x94, 0x8e, 0x88 },
      { 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0x09, 0x03, 0xc8, 0x88 },
      });

  credits->add_pattern({
      0x18,
      { 0x29, 0xec, 0xb0, 0x6d, 0xc2, 0x35, 0xf0, 0xb4, 0x37, 0x35, 0x29, 0xec, 0xb0, 0x35, 0xf0, 0xac, 0x67, 0x43, 0x43 },
      { 0x11, 0x11, 0x15, 0x15, 0x0f, 0x0f, 0x11, 0x0f, 0x11, 0x11, 0x15, 0xd0, 0x8e, 0x0f, 0x0f, 0x0b, 0x0f },
      { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x94, 0x94, 0x94, 0x94, 0x94, 0x94, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x98, 0x98, 0x98, 0x94, 0x98, 0x94, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x94, 0x94, 0x94, 0x94, 0x94, 0x94, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x90, 0x90, 0x90, 0x8e, 0x90, 0x8e },
      { 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x88, 0x88, 0x88 },
      });

  credits->add_pattern({
      0x18,
      { 0x29, 0xec, 0xb0, 0x6d, 0xc2, 0x35, 0xf0, 0xb4, 0x37, 0x37, 0x39, 0xf6, 0xb4, 0x31, 0xf4, 0xb8, 0x77, 0x43, 0x43 },
      { 0x11, 0x11, 0x15, 0x15, 0x0f, 0x0f, 0x19, 0x19, 0x11, 0x11, 0x19, 0xd8, 0x94, 0x1f, 0x1f, 0x1f, 0x03 },
      { 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x94, 0x94, 0x94, 0x94, 0x94, 0x94, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x98, 0x98, 0x98, 0x94, 0x98, 0x94, 0x90, 0x90, 0x90, 0x90, 0x90, 0x90, 0x94, 0x94, 0x94, 0x94, 0x94, 0x94, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x8e, 0x90, 0x90, 0x90, 0x8e, 0x90, 0x8e },
      { 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x09, 0x88, 0x88, 0x88 },
      });

  credits->set_sequence({0, 1, 0, 1, 2, 3});
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cout << "Usage: " << argv[0] << " z2_rom" << std::endl;
    return 1;
  }

  const std::string file = std::string(argv[1]);

  z2music::Rom rom(file);

  // Change gp music
  set_gp_intro(rom.song(z2music::Rom::SongTitle::GreatPalaceIntro));
  set_gp_theme(rom.song(z2music::Rom::SongTitle::GreatPalaceTheme));
  set_final_boss(rom.song(z2music::Rom::SongTitle::FinalBossTheme));
  set_zelda_theme(rom.song(z2music::Rom::SongTitle::ZeldaTheme));
  set_alt_credits_theme(rom.song(z2music::Rom::SongTitle::CreditsTheme));

  // Fix a bug in the vanilla game
  rom.write(0x5d6b, { 0xea, 0xea, 0xea });

  // Change credits
  z2music::Credits* c = rom.credits();
  c->set(0, {"WINTER SOLSTICE",   "BY GTM604",     ""});
  c->set(1, {"HACKING        ",   "CF207    ",     ""});
  c->set(2, {"MUSIC  ",           "BENTGLASSTUBE", ""});
  c->set(3, {"DESIGN",            "GTM604       ", "ICEPENGUIN"  });
  c->set(4, {"",                  "REP2369",       "MISTERMIKE"  });
  c->set(5, {"SPECIAL THANKS",    "MAX    ",       "SIMPOLDOOD"  });
  c->set(6, {"",                  "TRAX",          "NJOSRO    "  });
  c->set(7, {"",                  "ZELDA II",      "COMMUNITY"   });
  c->set(8, {"THANKS A MILLION/", "PUSH START TO", "KEEP FISHING"});

  rom.save("/tmp/output.nes");

  return 0;
}
