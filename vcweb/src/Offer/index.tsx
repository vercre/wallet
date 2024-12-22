import { useEffect, useState } from 'react';

import Box from '@mui/material/Box';
import Button from '@mui/material/Button';
import Grid from '@mui/material/Grid2';
import Stack from '@mui/material/Stack';
import Typography from '@mui/material/Typography';
import { useMutation } from '@tanstack/react-query';

import { createOffer } from '../api';
import CreateOffer from './CreateOffer';
import QrCode from './QrCode';
import FullLogo from '../components/FullLogo';
import { CreateOfferRequest } from '../types/generated';

const Offer = () => {
    const [processing, setProcessing] = useState<'EmployeeID_JWT' | 'Developer_JWT' | null>(null);
    const [pin, setPin] = useState<string>('');
    const [qrCode, setQrCode] = useState<string>('');

    // Effect to scroll back to top on reset
    useEffect(() => {
        if (processing === null) {
            document.getElementById('pageContent')?.scrollTo({
                top: 0,
                behavior: 'smooth',
            });
        }
    }, [processing]);

    // API call to create a credential offer
    const mut = useMutation({
        mutationFn: async (createOfferRequest: CreateOfferRequest) => {
            const response = await createOffer(createOfferRequest);
            setQrCode(response.qr_code);
            setPin(response.tx_code || '');
        }
    });

    const handleCreateOffer = async (configId: 'EmployeeID_JWT' | 'Developer_JWT') => {
        setProcessing(configId);
        const req: CreateOfferRequest = {
            // eslint-disable-next-line camelcase
            credential_issuer: 'http://vercre.io', // Gets ignored by the sample API.
            // eslint-disable-next-line camelcase
            subject_id: 'normal_user',
            // eslint-disable-next-line camelcase
            credential_configuration_id: configId,
            // eslint-disable-next-line camelcase
            grant_type: 'urn:ietf:params:oauth:grant-type:pre-authorized_code',
            // eslint-disable-next-line camelcase
            tx_code_required: true,
        };
        mut.mutate(req);
    };

    const handleReset = () => {
        setProcessing(null);
        setPin('');
    };

    return (
        <Stack spacing={4} py={4} id="pageContent">
            <Typography variant="h1">
                Credential Offer
            </Typography>
            {processing === null &&
                <Typography variant="body1">
                    Start the process of issuing a credential by choosing the credential type you would
                    like to issue. The user can then scan a QR code to accept the offer.
                </Typography>
            }
            <Grid container spacing={4}>
                <Grid size={{ xs: 12, sm: 6 }}>
                    {processing !== 'EmployeeID_JWT' &&
                        <CreateOffer
                            configId="EmployeeID_JWT"
                            disabled={processing !== null}
                            onCreate={() => handleCreateOffer('EmployeeID_JWT')}
                        />
                    }
                    {processing === 'EmployeeID_JWT' &&
                        <QrCode image={qrCode} pin={pin} />
                    }
                </Grid>
                <Grid size={{ xs: 12, sm: 6 }}>
                    {processing !== 'Developer_JWT' &&
                        <CreateOffer
                            configId="Developer_JWT"
                            disabled={processing !== null}
                        onCreate={() => handleCreateOffer('Developer_JWT')}
                        />
                    }
                    {processing === 'Developer_JWT' &&
                        <QrCode image={qrCode} pin={pin} />
                    }
                </Grid>
            </Grid>
            <Box sx={{ display: 'flex', justifyContent: 'center' }}>
                <Button
                    disabled={processing === null}
                    variant="contained"
                    color="secondary"
                    onClick={handleReset}
                    sx={{ maxWidth: '200px' }}
                >
                    Start Over
                </Button>
            </Box>
            <FullLogo />
        </Stack>
    );
};

export default Offer;