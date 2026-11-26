/**
 * SELF CONTAINED JAVA RANDOM TEST FILE
 * 
 * This exists to ensure that the randomization of Coffee does logically match
 * the randomization that the module 'java.util.Random' provides. This source file
 * is self contained and can be compiled and ran with the following command:
 * 
 * java java/RandomAssurance.java
 * 
 * SPDX-License-Identifier: GPL-3.0-only
 */

import java.util.Random;

class RandomAssurance {
    public static void main(String[] args) {

        Random rng1 = new Random(256090L);
        System.out.println("Seed outputs with seed '256090L':\n------------------------");
        for (int i = 0; i < 32; i++) {
            System.out.println(rng1.nextInt());
        }
        System.out.print("\n");

        Random rng2 = new Random(129176L);
        System.out.println("Seed outputs with seed '129176L':\n------------------------");
        for (int j = 0; j < 32; j++) {
            System.out.println(rng2.nextInt());
        }
        System.out.print("\n");

        Random rng3 = new Random(178962L);
        System.out.println("Seed outputs with seed '178962L':\n------------------------");
        for (int j = 0; j < 32; j++) {
            System.out.println(rng3.nextInt());
        }
        System.out.print("\n");

        Random rng4 = new Random(184400L);
        System.out.println("Seed outputs with seed '184400L':\n------------------------");
        for (int j = 0; j < 32; j++) {
            System.out.println(rng4.nextInt());
        }
        System.out.print("\n");

        System.out.println("------------------------\nAll data printed.");
        return;
    }
}