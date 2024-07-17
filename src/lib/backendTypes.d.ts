type PackItem = {
	name: string;
	location: string;
	quantity: number;
  packed: PackedItem[];
};

type PackedItem =
	| {
			type: 'generic';
	  }
	| {
			type: 'ephemeral';
			content: string;
	  }
	| {
			type: 'unique';
			content: string;
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

type UniqueItem = {
	name: string;
	description: string;
	location: string;
	thumbnail: string | null;
};

type PackingList = {
	name: string;
	items: ListItem[];
};
