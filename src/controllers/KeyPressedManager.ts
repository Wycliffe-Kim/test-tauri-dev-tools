/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */

import { KeyPressed } from '../types/KeyPressed';

export const KeyPressedManager = (wantToSet: Partial<KeyPressed>) => ({
  isDone: (event: KeyboardEvent) =>
    (wantToSet.alt === true ? event.altKey : true) &&
    (wantToSet.ctrl === true ? event.ctrlKey : true) &&
    (wantToSet.shift === true ? event.shiftKey : true) &&
    (wantToSet.meta === true ? event.metaKey : true) &&
    wantToSet.key?.toLocaleLowerCase() === event.key.toLowerCase(),
});
