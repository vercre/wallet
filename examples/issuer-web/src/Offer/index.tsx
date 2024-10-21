import { useState } from 'react';

import Grid from '@mui/material/Grid2';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';

import CreateOffer from './CreateOffer';
import FullLogo from '../components/FullLogo';
import QrCode from './QrCode';

const Offer = () => {
    const [processing, setProcessing] = useState<'EmployeeID_JWT' | 'Developer_JWT' | null>(null);

    return (
        <Stack spacing={4} py={4}>
            <Typography variant="h1">
                Credential Offer
            </Typography>
            <Typography variant="body1">
                Start the process of issuing a credential by choosing the credential type you would
                like to issue. The user can then scan a QR code to accept the offer.
            </Typography>
            <Grid container spacing={4}>
                <Grid size={{ xs: 12, sm: 6 }}>
                    {processing !== 'EmployeeID_JWT' &&
                        <CreateOffer
                            configId="EmployeeID_JWT"
                            disabled={processing !== null}
                            onCreate={() => setProcessing('EmployeeID_JWT')}
                        />
                    }
                    {processing === 'EmployeeID_JWT' &&
                        <QrCode image="https://example.com/qr-code.png" />
                    }
                </Grid>
                <Grid size={{ xs: 12, sm: 6 }}>
                    {processing !== 'Developer_JWT' &&
                        <CreateOffer
                            configId="Developer_JWT"
                            disabled={processing !== null}
                            onCreate={() => { }}
                        />
                    }
                    {processing === 'Developer_JWT' &&
                        <QrCode image="https://example.com/qr-code.png" />
                    }
                </Grid>
            </Grid>
            <FullLogo />
        </Stack>
    );
};

export default Offer;