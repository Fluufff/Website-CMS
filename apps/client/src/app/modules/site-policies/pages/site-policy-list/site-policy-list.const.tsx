import { Badge, Button, ButtonLink, ButtonSizes, ButtonTypes, ITableColumn } from '@ibs/components';
import { IIAMPolicy, IPermission } from '@ibs/shared';
import { TFunction } from 'i18next';

export const POLICY_LIST_COLUMNS = (t: TFunction, handleRemove: (policyId: string) => void): ITableColumn<IIAMPolicy>[] => [
	{
		id: 'name',
		label: 'Name',
	},
	{
		id: 'permissions',
		label: 'Permissions',
		format: (permissions: IPermission[]) => permissions.map((permission, i) => <Badge className='u-margin-right-xs' key={i}>{permission.effect}: {permission.resources.join(', ')} / {permission.actions.join(', ')}</Badge>)
	},
	{
		id: 'actions',
		label: '',
		format: (value, key, item) => (
			<div className="u-display-flex">
				<ButtonLink to={`${item.id}`} size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-auto">
					<i className="las la-pen"></i> {t(`GENERAL.LABELS.EDIT`)}
				</ButtonLink>
				<Button size={ButtonSizes.SMALL} type={ButtonTypes.SECONDARY} className="u-margin-left-sm" onClick={() => handleRemove(item.id)}>
					<i className="las la-trash"></i>
				</Button>
			</div>
		),
	},
];
