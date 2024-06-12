/** @type {import('tailwindcss').Config} */
export const content = ["./../../../src/**/*.rs"];
export const theme = {
  extend: {},
};
export const plugins = [
  require('@tailwindcss/typography'),
  require('daisyui'),
];