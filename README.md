# 7-loaders
a basic data analyzing tool for 7 wonders board game

enable statistic visualization of post-game scores using a .txt (currently only parses the txt, data visualization & calculation still to be added)

to use, fill a txt (or ask your friendly libreOffice Calc to do it for you) with the supported format (a demo txt is provided on the github):
  -base format for a single game is :
	
		"NAME_OF_WONDER"/"SIDE_OF_WONDER"/"WONDER_POINTS"/"MONEY_POINTS"/"ARMY_POINTS"/"BLUE_POINTS"/"YELLOW_POINTS"/"GREEN_POINTS"/"PURPLE_POINTS"
		"NAME_OF_WONDER"/"SIDE_OF_WONDER"/"WONDER_POINTS"/"MONEY_POINTS"/"ARMY_POINTS"/"BLUE_POINTS"/"YELLOW_POINTS"/"GREEN_POINTS"/"PURPLE_POINTS"
		etc.. for each player
    
  -base format to visualize multiple single games a once is :
	
		"GAME_ONE".
		"GAME_TWO".
		"GAME_THREE"
