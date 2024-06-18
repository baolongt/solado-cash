
    'use client';


import { useWallet } from '@solana/wallet-adapter-react';
import { ExplorerLink } from '../cluster/cluster-ui';
import { WalletButton } from '../solana/solana-provider';
import { AppHero, ellipsify } from '../ui/ui-layout';
import { useSoladoCashProgram } from './solado-cash-data-access';
import { SoladoCashCreate, SoladoCashProgram } from './solado-cash-ui';

export default function SoladoCashFeature() {
  const { publicKey } = useWallet();
  const { programId } = useSoladoCashProgram();

  return publicKey ? (
    <div>
      <AppHero title="SoladoCash" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
        </p>
        <SoladoCashCreate />
      </AppHero>
      <SoladoCashProgram />
    </div>
  ) : (
    <div className="max-w-4xl mx-auto">
      <div className="hero py-[64px]">
        <div className="hero-content text-center">
          <WalletButton className="btn btn-primary" />
        </div>
      </div>
    </div>
  );
}
