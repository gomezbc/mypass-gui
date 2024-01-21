import { type Credential } from "@/types/Credential";

export type Login = {
  domain: string;
  credentials: Credential[];
};
