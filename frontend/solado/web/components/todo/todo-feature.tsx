
    'use client';


import { useWallet } from '@solana/wallet-adapter-react';
import { ExplorerLink } from '../cluster/cluster-ui';
import { WalletButton } from '../solana/solana-provider';
import { AppHero, ellipsify } from '../ui/ui-layout';
import { useTodoProgram } from './todo-data-access';
import { TodoCreate, TodoProgram } from './todo-ui';

export default function TodoFeature() {
  const { publicKey } = useWallet();
  const { programId } = useTodoProgram();

  return publicKey ? (
    <div>
      <AppHero title="Todo" subtitle={'Run the program by clicking the "Run program" button.'}>
        <p className="mb-6">
          <ExplorerLink path={`account/${programId}`} label={ellipsify(programId.toString())} />
        </p>
        <TodoCreate />
      </AppHero>
      <TodoProgram />
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
