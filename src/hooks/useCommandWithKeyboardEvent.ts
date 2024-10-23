/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */
import { os } from '@tauri-apps/api';
import { Either } from 'monet';
import { tap } from 'ramda';
import { useEffect } from 'react';
import { fromEvent } from 'rxjs';
import { KeyPressed } from '../types/KeyPressed';
import { KeyPressedManager } from '../controllers/KeyPressedManager';
import { OsType } from '../types/OsType';

type Params = { command: () => Promise<unknown> } & Partial<{
  name: string;
  windows: Partial<KeyPressed>;
  macos: Partial<KeyPressed>;
  linux: Partial<KeyPressed>;
}>;

export const useCommandWithKeyboardEvent = ({
  command,
  name,
  linux,
  macos,
  windows,
}: Params) => {
  const windowsKeyPressedManager = KeyPressedManager(windows ?? {});
  const macKeyPressedManager = KeyPressedManager(macos ?? {});
  const linuxKeyPressedManager = KeyPressedManager(linux ?? {});

  useEffect(() => {
    const subscriptionForKeyDown = fromEvent<KeyboardEvent>(
      window,
      'keydown'
    ).subscribe((event) => {
      Either.fromPromise(os.type()).then((either) =>
        either
          .map((type) => OsType.fromTauriOsType(type))
          .map(
            tap((type) => {
              if (
                type === 'MACOS'
                  ? macKeyPressedManager.isDone(event)
                  : type === 'WINDOWS'
                  ? windowsKeyPressedManager.isDone(event)
                  : linuxKeyPressedManager.isDone(event)
              ) {
                Either.fromPromise(command()).then((either) =>
                  either.leftMap((error) =>
                    console.error(
                      `[useCommandWithKeyboardEvent]${
                        name !== undefined ? ` ${name}` : ''
                      }`,
                      error
                    )
                  )
                );
              }
            })
          )
      );
    });

    return () => {
      subscriptionForKeyDown.unsubscribe();
    };
  }, []);
};
