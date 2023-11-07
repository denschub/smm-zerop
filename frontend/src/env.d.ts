/// <reference types="astro/client" />

interface ImportMetaEnv {
  readonly PUBLIC_SMM_ZEROP_API_ROOT: string;
}

interface ImportMeta {
  readonly env: ImportMetaEnv;
}
