// src/echarts/themes.ts
import light from "@/theme/echarts/light.json";
import { registerTheme } from "echarts";
import VChart from "vue-echarts";

registerTheme("light", light);

VChart.props.theme.default = "light";
