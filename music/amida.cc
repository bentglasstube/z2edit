#include "music.h"

#include <iostream>

void set_title_intro(z2music::Song *title) {
}

void set_title_theme_buildup(z2music::Song *title) {
}

void set_title_theme_main(z2music::Song *title) {
}

void set_title_theme_breakdown(z2music::Song *title) {
}

void set_overworld_intro(z2music::Song* intro) {
}

void set_overworld_theme(z2music::Song* theme) {
}

void set_encounter_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x18,
      { 0xa8, 0xa4, 0xa8, 0xac, 0x6f, 0x82, 0xac, 0xae, 0xb2, 0xb6, 0xb2, 0xae, 0xac, 0x6f, 0x6d, 0xee, 0xec, 0xe8, 0xec, 0x65, 0x43, 0xde, 0xe4, 0xe8, 0xec, 0x69, 0x43, 0x6f, 0x6d },
      { 0x43, 0x90, 0x8c, 0x90, 0x94, 0x96, 0x03, 0x43, 0x9a, 0x96, 0x94, 0x82, 0x43, 0x96, 0x94, 0x90, 0x03, 0x43, 0xd6, 0xd4, 0xcc, 0xc6, 0xd0, 0xcc, 0xd0, 0x8c, 0x90, 0x94, 0x96, 0x94, 0x43, 0x03, 0x96, 0x94, 0x90, 0x8c },
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x8c, 0xa4, 0x8c, 0xa4 },
      { 0xc2, 0x49, 0x88, 0x09, 0x49, 0x88, 0x09, 0x49, 0x88, 0x09, 0x09, 0x88, 0x88, 0x88 },
      });

  theme->add_pattern({
      0x18,
      { 0xa8, 0xa4, 0xa8, 0xac, 0x6f, 0x82, 0xac, 0xae, 0xb2, 0xb6, 0xb2, 0xae, 0xac, 0x6f, 0x6d, 0xe8, 0xec, 0xee, 0xf2, 0x77, 0x43, 0xfa, 0xf6, 0xf2, 0xee, 0x69, 0x43, 0x6f, 0x73 },
      { 0x43, 0x90, 0x8c, 0x90, 0x94, 0x96, 0x03, 0x43, 0x9a, 0x96, 0x94, 0x82, 0x43, 0x96, 0x94, 0x90, 0x03, 0x43, 0xe8, 0xe4, 0xe0, 0xde, 0xda, 0xd6, 0xda, 0x8c, 0x90, 0x94, 0x96, 0x94, 0x43, 0x03, 0x96, 0x94, 0x90, 0x94 },
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x90, 0xa8, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x88, 0xa0, 0x8c, 0xa4, 0x8c, 0xa4 },
      { 0xc2, 0x49, 0x88, 0x09, 0x49, 0x88, 0x09, 0x49, 0x88, 0x09, 0x09, 0x88, 0x88, 0x88 },
      });

  theme->set_sequence({0, 1});
}

void set_town_intro(z2music::Song* intro) {
  intro->clear();

  intro->add_pattern({
      0x20,
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  intro->set_sequence({0});
}

void set_town_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x6f, 0x03, 0xee, 0x6d, 0xc2, 0xec, 0xe8, 0xac, 0xae, 0x82, 0xac, 0xe8 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xcc, 0x88, 0x0d, 0x07, 0x09, 0xe4 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x6f, 0x03, 0xee, 0x6d, 0xc2, 0xec, 0xa8, 0xac, 0xae, 0xb2, 0x82, 0xb6, 0xf2 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xcc, 0x88, 0x0d, 0xc6, 0x88, 0xcc, 0x88, 0x86, 0x88 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x6f, 0x03, 0xee, 0x6d, 0xc2, 0xec, 0xa8, 0xa4, 0xa0, 0x9e, 0xa0, 0x9e, 0x9a, 0x96 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xcc, 0x88, 0x0d, 0xc6, 0x88, 0xcc, 0x88, 0x86, 0x88 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x5f, 0x03, 0xde, 0x5b, 0xc2, 0xda, 0x96, 0x82, 0x9a, 0x9e, 0x82, 0x9a, 0xd6 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xd0, 0x8c, 0x11, 0xd6, 0x94, 0xd0, 0x0d },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x5f, 0x03, 0xde, 0x5b, 0xc2, 0xda, 0x96, 0x9a, 0x9e, 0xa0, 0x82, 0xa4, 0xe0 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xd0, 0x8c, 0x11, 0xd6, 0x94, 0xd0, 0x94, 0xd6 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->add_pattern({
      0x20,
      { 0xc2, 0xa8, 0x82, 0x9e, 0xa8, 0x82, 0x5f, 0x03, 0xde, 0x5b, 0xc2, 0xda, 0xd6, 0x9a, 0x96, 0x82, 0x94, 0xd0 },
      { 0x43, 0x43, 0xc2, 0xd0, 0x8c, 0x11, 0xc2, 0xd0, 0x8c, 0x11, 0xd6, 0x94, 0xd0, 0x94, 0xd6 },
      { 0x90, 0xc2, 0x90, 0xc2, 0x96, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9a, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x9e, 0x82, 0x90, 0xc2, 0x90, 0xc2, 0x8c, 0x82 },
      { 0x88, 0x88, 0x88, 0xc8, 0x88, 0xc8 },
      });

  theme->set_sequence({3, 4, 3, 5, 0, 1, 0, 2});
}

void set_ruins_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x10,
      { 0x5d, 0x43, 0xc2, 0xd8, 0xdc, 0xde, 0x63, 0x5f, 0x5d, 0x59 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0xa6, 0xa0, 0x9a, 0x94, 0xa6, 0xa0, 0x9a, 0x94, 0xa6, 0xa0, 0x9a, 0x94, 0xa6, 0xa0, 0x9a, 0x94, 0xa2, 0x9c, 0x96, 0x90, 0xa2, 0x9c, 0x96, 0x90, 0xa2, 0x9c, 0x96, 0x90, 0xa2, 0x9c, 0x96, 0x90 },
      { 0x49, 0x49, 0xc0, 0xc8, 0x88, 0x09 },
      });

  theme->add_pattern({
      0x10,
      { 0x55, 0x43, 0xc2, 0xd2, 0xd4, 0xd8, 0x4f, 0x43, 0x53, 0x43 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9a, 0x94, 0x8e, 0x88, 0x9a, 0x94, 0x8e, 0x88, 0x9a, 0x94, 0x8e, 0x88, 0x88, 0x8e, 0x94, 0x9a },
      { 0x49, 0x49, 0xc0, 0xc8, 0x88, 0x09 },
      });

  theme->add_pattern({
      0x10,
      { 0x55, 0x43, 0xc2, 0xd2, 0xd4, 0xd8, 0x5d, 0x59, 0x63, 0x5f },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9e, 0x98, 0x92, 0x8c, 0x9a, 0x94, 0x8e, 0x88, 0x9a, 0x94, 0x8e, 0x88, 0x9a, 0x94, 0x8e, 0x88, 0x8e, 0x94, 0x9a, 0xa0 },
      { 0x49, 0x49, 0xc0, 0xc8, 0x88, 0x09 },
      });

  theme->set_sequence({0, 0, 1, 2});
}

void set_dungeon_intro(z2music::Song* intro) {
}

void set_dungeon_theme(z2music::Song* theme) {
}

void set_boss_theme(z2music::Song* theme) {
}

void set_great_palace_intro(z2music::Song* intro) {
}

void set_great_palace_theme(z2music::Song* theme) {
}

void set_final_boss_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x18,
      { 0x82, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0x37, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0x37, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0xb6, 0xb8, 0xbc, 0xb8, 0xb6, 0xb4, 0xb6, 0xb4, 0xb0, 0xae },
      { 0x19, 0x19, 0xd8, 0x17, 0x17, 0xd6, 0x15, 0x15, 0xd4, 0x13, 0x13, 0xd2 },
      { 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2 },
      { 0xc8, 0x49, 0x49, 0x49, 0x88, 0x88 },
      });

  theme->add_pattern({
      0x18,
      { 0x82, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0x37, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0x37, 0xb0, 0xb0, 0xae, 0xb0, 0xb4, 0xb6, 0xb8, 0xbc, 0xb8, 0xb6, 0xb8, 0xfc, 0xf8 },
      { 0x19, 0x19, 0xd8, 0x17, 0x17, 0xd6, 0x15, 0x15, 0xd4, 0x13, 0x13, 0xd2 },
      { 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2 },
      { 0xc8, 0x49, 0x49, 0x49, 0x88, 0x88 },
      });

  theme->add_pattern({
      0x18,
      { 0xc2, 0xf0, 0xb4, 0xf6, 0x39, 0xf6, 0xf8, 0x7d, 0xf0, 0xb4, 0xf6, 0x39, 0xf6, 0x75 },
      { 0x19, 0x19, 0xd8, 0x17, 0x17, 0xd6, 0x15, 0x15, 0xd4, 0x13, 0x13, 0xd2 },
      { 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2 },
      { 0xc8, 0x49, 0x49, 0x49, 0x88, 0x88 },
      });

  theme->add_pattern({
      0x18,
      { 0xc2, 0xf0, 0xb4, 0xf6, 0x79, 0x37, 0xf8, 0x3d, 0x39, 0xf6, 0xf8, 0xf6, 0xf4, 0xee },
      { 0x19, 0x19, 0xd8, 0x17, 0x17, 0xd6, 0x15, 0x15, 0xd4, 0x13, 0x13, 0xd2 },
      { 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x98, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9c, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0x9e, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2, 0xa2 },
      { 0xc8, 0x49, 0x49, 0x49, 0x88, 0x88 },
      });

  theme->set_sequence({0, 1, 0, 1, 2, 3});
}

void set_zelda_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x20,
      { 0x8e, 0x03, 0x94, 0x82, 0x8a, 0x03, 0x86, 0x8a, 0x8e, 0x03, 0x94, 0x82, 0x8a, 0x82, 0x43, 0x8e, 0x03, 0x94, 0x82, 0xa2, 0x03, 0x9e, 0x82, 0x94, 0x03, 0x90, 0x8e, 0x8a, 0x82, 0x43 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x90, 0x9e, 0xb6, 0x03, 0x90, 0xa2, 0xb4, 0x03, 0x90, 0x9e, 0xb6, 0x03, 0x90, 0xa2, 0xb4, 0x03, 0x8e, 0xac, 0xb8, 0x03, 0x8c, 0xaa, 0xb6, 0x03, 0x8a, 0xa8, 0xb6, 0x03, 0x94, 0xa2, 0xb4, 0x03 },
      {},
      });

  theme->add_pattern({
      0x20,
      { 0x8e, 0x03, 0x94, 0x82, 0x8a, 0x03, 0x86, 0x8a, 0x8e, 0x03, 0x94, 0x82, 0x8a, 0x82, 0x43, 0x8e, 0x03, 0x94, 0x82, 0xa2, 0x03, 0x9e, 0x82, 0xac, 0x03, 0x43, 0xc2, 0x1e, 0x22, 0x26, 0x28 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x90, 0x9e, 0xb6, 0x03, 0x90, 0xa2, 0xb4, 0x03, 0x90, 0x9e, 0xb6, 0x03, 0x90, 0xa2, 0xb4, 0x03, 0x8e, 0xac, 0xb8, 0x03, 0x8c, 0xaa, 0xb6, 0x03, 0x8a, 0xa8, 0xb6, 0x03, 0x94, 0xa2, 0xb4, 0x03 },
      {},
      });

  theme->add_pattern({
      0x20,
      { 0xac, 0x03, 0xa8, 0xa6, 0xa8, 0xa6, 0x9e, 0x03, 0xa8, 0x03, 0xa6, 0xa2, 0xa6, 0xa2, 0x98, 0x03 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43 },
      { 0x9a, 0xa2, 0xa8, 0xb0, 0xa8, 0xa2, 0x98, 0x9e, 0xa6, 0xac, 0xa6, 0x9e, 0x94, 0x9a, 0xa2, 0xa8, 0xa2, 0x9a, 0x90, 0x98, 0x9e, 0xa6, 0x9e, 0x98 },
      {},
      });

  theme->add_pattern({
      0x20,
      { 0xac, 0x03, 0xa8, 0xa6, 0xa8, 0xa6, 0x9e, 0x82, 0xa8, 0x82, 0xb6, 0x03, 0x43, 0x43, 0x43, 0xc2 },
      { 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0x43, 0xc2 },
      { 0x9a, 0xa2, 0xa8, 0xb0, 0xa8, 0xa2, 0x98, 0x9e, 0xa6, 0xac, 0xa6, 0x9e, 0x94, 0x9e, 0xa4, 0xaa, 0xa4, 0x9e, 0xa2, 0xa8, 0xb2, 0xb8, 0xac, 0xbc, 0x43, 0xc2 },
      {},
      });

  theme->set_sequence({0, 1, 2, 3});
}

void set_credits_theme(z2music::Song* theme) {
  theme->clear();

  theme->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xd0, 0x1f, 0x1b, 0xd8, 0x1b, 0x19, 0xd0},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xf0, 0xec, 0xb0, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  theme->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xd0, 0x15, 0x19, 0xd0, 0x15, 0x11, 0xce},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xec, 0xe6, 0xac, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  theme->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xde, 0x1b, 0x19, 0xd0, 0xd4, 0xd8, 0xda, 0xd8},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xf0, 0xec, 0xb0, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  theme->add_pattern({
      0x28,
      { 0x1b, 0x19, 0xd0, 0x1b, 0x19, 0xde, 0x1b, 0x19, 0xd0, 0xd4, 0xd0, 0xce, 0xd0},
      { 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xc2, 0xe8, 0xa8, 0x2d, 0xf2, 0xec, 0xe6, 0xac, 0xa8},
      { 0x90, 0xa8, 0x90, 0xa8, 0x90, 0x28, 0x90, 0x10, 0xa8, 0x8a, 0xa2, 0x8a, 0xa2, 0x8a, 0x22, 0x8a, 0x0a, 0xa2, 0x8c, 0xa4, 0x8c, 0xa4, 0x8c, 0x24, 0x8c, 0x0c, 0xa4, 0x86, 0x9e, 0x86, 0x9e, 0x88, 0x20, 0x8a, 0x0a, 0xa2},
      { 0x49, 0x49, 0x49, 0x48, 0x48, 0x88},
      });

  theme->set_sequence({0, 1, 0, 1, 2, 3});
}

void set_fanfare(z2music::Song *fanfare) {
  fanfare->clear();

  fanfare->add_pattern({
      0x10,
      { 0xa0, 0xa2, 0xa4, 0x27 },
      { 0x9a, 0x9c, 0x9e, 0x21 },
      { 0x9a, 0x9c, 0x9e, 0x21 },
      {},
      });

  fanfare->set_sequence({0});
}

int main(int argc, char** argv) {
  if (argc != 2) {
    std::cout << "Usage: " << argv[0] << " z2_rom" << std::endl;
    return 1;
  }

  const std::string file = std::string(argv[1]);

  z2music::Rom rom(file);

  // Change title music
  set_title_intro(rom.song(z2music::Rom::SongTitle::TitleIntro));
  set_title_theme_buildup(rom.song(z2music::Rom::SongTitle::TitleThemeStart));
  set_title_theme_main(rom.song(z2music::Rom::SongTitle::TitleThemeBuildup));
  set_title_theme_main(rom.song(z2music::Rom::SongTitle::TitleThemeMain));
  set_title_theme_breakdown(rom.song(z2music::Rom::SongTitle::TitleThemeBreakdown));

  // Change overworld music
  set_overworld_intro(rom.song(z2music::Rom::SongTitle::OverworldIntro));
  set_overworld_theme(rom.song(z2music::Rom::SongTitle::OverworldTheme));
  set_encounter_theme(rom.song(z2music::Rom::SongTitle::BattleTheme));
  set_town_intro(rom.song(z2music::Rom::SongTitle::TownIntro));
  set_town_theme(rom.song(z2music::Rom::SongTitle::TownTheme));
  set_ruins_theme(rom.song(z2music::Rom::SongTitle::HouseTheme));

  // Change dungeon music
  set_dungeon_intro(rom.song(z2music::Rom::SongTitle::PalaceIntro));
  set_dungeon_theme(rom.song(z2music::Rom::SongTitle::PalaceTheme));
  set_boss_theme(rom.song(z2music::Rom::SongTitle::BossTheme));

  // Change great palace music
  set_great_palace_intro(rom.song(z2music::Rom::SongTitle::GreatPalaceIntro));
  set_great_palace_theme(rom.song(z2music::Rom::SongTitle::GreatPalaceTheme));
  set_final_boss_theme(rom.song(z2music::Rom::SongTitle::FinalBossTheme));

  // Change credits music
  set_zelda_theme(rom.song(z2music::Rom::SongTitle::ZeldaTheme));
  set_credits_theme(rom.song(z2music::Rom::SongTitle::CreditsTheme));

  set_fanfare(rom.song(z2music::Rom::SongTitle::CaveItemFanfare));
  set_fanfare(rom.song(z2music::Rom::SongTitle::TownItemFanfare));
  set_fanfare(rom.song(z2music::Rom::SongTitle::PalaceItemFanfare));
  set_fanfare(rom.song(z2music::Rom::SongTitle::GreatPalaceItemFanfare));

  // Fix a bug in the vanilla game
  rom.write(0x5d6b, { 0xea, 0xea, 0xea });

  // Change credits
  z2music::Credits* c = rom.credits();
  c->set(0, {"CREATED BY",      "OK IMPALA!",     ""});
  c->set(1, {"Z2 REDUX  ",      "SHAODWONE333",   ""});
  c->set(2, {"GRAPHICS",        "REVILITY    ",   "GTM"});
  c->set(3, {"",                "FALCHION22",     "OK IMPALA!"});
  c->set(4, {"MUSIC",           "BENTGLASSTUBE",  "          "});
  c->set(5, {"SPECIAL THANKS",  "NJOSRO       ",  "TRAX"});
  c->set(6, {"",                "CFRANTZ",        "MELCHIOR"});
  c->set(7, {"",                "DAAN   ",        "GTM     "});
  c->set(8, {"OKIMPALA.NET  ",  "PUSH START",     "TO REPLAY"});

  rom.save("/tmp/output.nes");

  return 0;
}
