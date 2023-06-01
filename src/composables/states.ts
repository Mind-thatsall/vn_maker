import { useState } from "nuxt/app";
import { sceneImagesType } from "~/types/types";

export const useImages = () =>
  useState<Array<sceneImagesType>>(
    "images",
    () => new Array<sceneImagesType>()
  );

export const useCounterForScene = () => useState("counter", () => 0);
