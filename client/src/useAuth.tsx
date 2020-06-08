import {createContext, Dispatch, SetStateAction} from "react";

export const AuthContext = createContext<{ authToken?: string|null, setAuthToken?: Dispatch<SetStateAction<string|null>> }>({});