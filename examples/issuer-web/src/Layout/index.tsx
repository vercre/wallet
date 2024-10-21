import Box from '@mui/material/Box';
import Container from '@mui/material/Container';
import Stack from '@mui/material/Stack';
import Toolbar from '@mui/material/Toolbar';
import { Outlet } from 'react-router-dom';
import { atom, useRecoilValue } from 'recoil';

import Footer from './Footer';
import Header, { HeaderProps } from './Header';

export const headerState = atom<HeaderProps>({
    key: 'header',
    default: {
        title: 'Credential Issuer',
        action: undefined,
        secondaryAction: undefined,
    },
});

const Layout = () => {
    const headerProps = useRecoilValue(headerState);
    return (
        <Stack
            sx={{
                flexGrow: 1,
                minHeight: '100vh',
            }}
        >
            <Header { ...headerProps } />
            <Container maxWidth="lg">
                <Box sx={{ flexGrow: 1 }}>
                    <Toolbar />
                    <Outlet />
                    <Toolbar />
                </Box>
            </Container>
            <Footer />
        </Stack>
    );
};

export default Layout;