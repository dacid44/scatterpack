type PackItem = {
	name: string;
	location: string;
	quantity: number;
};

type PackCollection = {
	name: string;
	items: ListItem[];
};

type ListItem =
	| {
			type: 'item';
			content: PackItem;
	  }
	| {
			type: 'collection';
			content: PackCollection;
	  };

type PackingList = {
	name: string;
	items: ListItem[];
};
