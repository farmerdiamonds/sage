import Container from '@/components/Container';
import Header from '@/components/Header';
import Layout from '@/components/Layout';
import { Trans } from '@lingui/react/macro';
import { t } from '@lingui/core/macro';
import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
  } from '@/components/ui/card';

export default function AntiCounterfeit() {
  return (
    <Layout>
      <Header title={t`Anti-counterfeit`} />

      <Container className='flex flex-col gap-4 max-w-screen-lg'>
        <Card>
          <CardHeader>
            <CardTitle className='text-lg font-medium'>
              <Trans>Anit-Counterfeit</Trans>
            </CardTitle>
            <CardDescription>
              <Trans>
                Prove or secure the authenticity of your product.
              </Trans>
            </CardDescription>
          </CardHeader>
          <CardContent>
            <Trans>
              This feature allows you to verify and secure the authenticity of products using blockchain technology.
            </Trans>
          </CardContent>
        </Card>
      </Container>
    </Layout>
  );
}
