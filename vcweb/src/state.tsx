import { atom } from "recoil";

import { HeaderProps } from "./Layout/Header";

export const headerState = atom<HeaderProps>({
    key: 'header',
    default: {
        title: 'Credential Issuer',
        action: undefined,
        secondaryAction: undefined,
    },
});
