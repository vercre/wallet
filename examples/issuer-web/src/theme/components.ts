export const components = {
    MuiButton: {
        styleOverrides: {
            contained: {
                borderRadius: '20px',
                boxShadow: 'none',
            },
            containedSecondary: {
                color: '#ffffff',
            },
            outlined: {
                borderRadius: '20px',
            },
            root: {
                '&:hover': {
                    boxShadow: 'none',
                },
            },
        },
    },
};
